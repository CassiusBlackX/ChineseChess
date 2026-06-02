import init, { WasmGame } from "../../pkg/gomoku/gomoku.js";

const statusEl = document.querySelector("#status");
const boardEl = document.querySelector("#board");
const resetEl = document.querySelector("#reset");
const playModeEl = document.querySelector("#play-mode");
const aiDifficultyEl = document.querySelector("#ai-difficulty");
const humanSideEl = document.querySelector("#human-side");

let game;
let syncingControls = false;

function keyOf(x, y) {
  return `${x},${y}`;
}

function pieceMap(pieces) {
  const map = new Map();
  for (const p of pieces) {
    map.set(keyOf(p.x, p.y), p);
  }
  return map;
}

function playModeLabel(mode) {
  return mode === "HumanVsAi" ? "人机对战" : "人人对战";
}

function difficultyLabel(level) {
  switch (level) {
    case "Easy":
      return "简单";
    case "Hard":
      return "困难";
    default:
      return "中等";
  }
}

function formatSession(session) {
  if (!session) {
    return "";
  }
  const mode = playModeLabel(session.play_mode);
  const difficulty = difficultyLabel(session.ai_difficulty);
  const human = session.human_side > 0 ? "玩家执黑" : "玩家执白";
  return ` | ${mode} · ${difficulty} · ${human}`;
}

function syncControls(snapshot) {
  const session = snapshot.session;
  if (!session) {
    return;
  }

  syncingControls = true;
  playModeEl.value = session.play_mode === "HumanVsAi" ? "pve" : "pvp";
  aiDifficultyEl.disabled = session.play_mode !== "HumanVsAi";

  switch (session.ai_difficulty) {
    case "Easy":
      aiDifficultyEl.value = "easy";
      break;
    case "Hard":
      aiDifficultyEl.value = "hard";
      break;
    default:
      aiDifficultyEl.value = "medium";
      break;
  }

  humanSideEl.value = session.human_side > 0 ? "black" : "white";
  syncingControls = false;
}

function render(snapshot) {
  const width = snapshot.width ?? 15;
  const height = snapshot.height ?? 15;
  const turnText = snapshot.turn > 0 ? "黑方" : "白方";
  let gameStateText = "";
  if (snapshot.game_over) {
    if (snapshot.winner === 0) {
      gameStateText = " | 对局结束: 和棋";
    } else {
      gameStateText = ` | 对局结束: ${snapshot.winner > 0 ? "黑方" : "白方"}胜`;
    }
  }
  statusEl.textContent = `${snapshot.message} | 当前回合: ${turnText}${formatSession(snapshot.session)}${gameStateText}`;

  syncControls(snapshot);

  const pieces = pieceMap(snapshot.pieces);
  const lastMove = snapshot.last_move ? keyOf(snapshot.last_move.x, snapshot.last_move.y) : null;
  const inputEnabled = snapshot.session?.human_input_enabled ?? true;
  const boardDisabled = Boolean(snapshot.game_over) || !inputEnabled;

  boardEl.style.gridTemplateColumns = `repeat(${width}, 1fr)`;
  boardEl.style.gridTemplateRows = `repeat(${height}, 1fr)`;

  boardEl.innerHTML = "";
  for (let y = height - 1; y >= 0; y -= 1) {
    for (let x = 0; x < width; x += 1) {
      const k = keyOf(x, y);
      const cell = document.createElement("button");
      cell.className = "cell";
      cell.disabled = boardDisabled;
      if (lastMove === k) {
        cell.classList.add("last-move");
      }

      const piece = pieces.get(k);
      if (piece) {
        const token = document.createElement("span");
        token.className = `stone ${piece.side > 0 ? "black" : "white"}`;
        cell.appendChild(token);
      }

      if (!boardDisabled) {
        cell.addEventListener("click", async () => {
          const next = await game.click(x, y);
          render(next);
        });
      }

      boardEl.appendChild(cell);
    }
  }
}

async function bootstrap() {
  await init();
  game = new WasmGame();
  render(await game.snapshot());

  resetEl.addEventListener("click", async () => {
    game.reset();
    render(await game.snapshot());
  });

  playModeEl.addEventListener("change", async () => {
    if (syncingControls) {
      return;
    }
    render(await game.set_play_mode(playModeEl.value));
  });

  aiDifficultyEl.addEventListener("change", async () => {
    if (syncingControls) {
      return;
    }
    render(await game.set_ai_difficulty(aiDifficultyEl.value));
  });

  humanSideEl.addEventListener("change", async () => {
    if (syncingControls) {
      return;
    }
    render(await game.set_human_side(humanSideEl.value));
  });
}

bootstrap().catch((err) => {
  statusEl.textContent = `初始化失败: ${err}`;
});
