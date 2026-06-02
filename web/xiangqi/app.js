import init, { WasmGame } from "../../pkg/xiangqi/xiangqi.js";

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

function moveSet(moves) {
  const set = new Set();
  for (const m of moves) {
    set.add(keyOf(m.x, m.y));
  }
  return set;
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
  const human = session.human_side > 0 ? "玩家执红" : "玩家执黑";
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

  humanSideEl.value = session.human_side > 0 ? "red" : "black";
  syncingControls = false;
}

function render(snapshot) {
  const width = snapshot.width ?? 9;
  const height = snapshot.height ?? 10;
  const turnText = snapshot.turn > 0 ? "红方" : "黑方";
  let gameStateText = "";
  if (snapshot.game_over) {
    gameStateText = ` | 对局结束: ${snapshot.winner > 0 ? "红方" : "黑方"}胜`;
  } else if (snapshot.in_check_side) {
    const checked = snapshot.in_check_side > 0 ? "红方" : "黑方";
    gameStateText = ` | 被将军: ${checked}`;
  }
  statusEl.textContent = `${snapshot.message} | 当前回合: ${turnText}${formatSession(snapshot.session)}${gameStateText}`;

  syncControls(snapshot);

  const pieces = pieceMap(snapshot.pieces);
  const moves = moveSet(snapshot.legal_moves);
  const selected = snapshot.selected ? keyOf(snapshot.selected.x, snapshot.selected.y) : null;
  const inputEnabled = snapshot.session?.human_input_enabled ?? true;
  const boardDisabled = Boolean(snapshot.game_over) || !inputEnabled;

  boardEl.style.gridTemplateColumns = `repeat(${width}, 1fr)`;
  boardEl.style.gridTemplateRows = `repeat(${height}, 1fr)`;
  boardEl.style.aspectRatio = `${width} / ${height}`;

  boardEl.innerHTML = "";
  for (let y = height - 1; y >= 0; y -= 1) {
    for (let x = 0; x < width; x += 1) {
      const k = keyOf(x, y);
      const cell = document.createElement("button");
      cell.className = "cell";
      cell.disabled = boardDisabled;
      cell.dataset.x = String(x);
      cell.dataset.y = String(y);
      if (selected && selected === k) {
        cell.classList.add("selected");
      }
      if (moves.has(k)) {
        cell.classList.add("hint");
      }

      const piece = pieces.get(k);
      if (piece) {
        const token = document.createElement("span");
        token.className = `piece ${piece.side > 0 ? "red" : "black"}`;
        token.textContent = piece.symbol;
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
