import init, { WasmGame } from "../../pkg/gomoku/gomoku.js";

const statusEl = document.querySelector("#status");
const boardEl = document.querySelector("#board");
const resetEl = document.querySelector("#reset");

let game;

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
  statusEl.textContent = `${snapshot.message} | 当前回合: ${turnText}${gameStateText}`;

  const pieces = pieceMap(snapshot.pieces);
  const lastMove = snapshot.last_move ? keyOf(snapshot.last_move.x, snapshot.last_move.y) : null;

  boardEl.style.gridTemplateColumns = `repeat(${width}, 1fr)`;
  boardEl.style.gridTemplateRows = `repeat(${height}, 1fr)`;

  boardEl.innerHTML = "";
  for (let y = height - 1; y >= 0; y -= 1) {
    for (let x = 0; x < width; x += 1) {
      const k = keyOf(x, y);
      const cell = document.createElement("button");
      cell.className = "cell";
      cell.disabled = Boolean(snapshot.game_over);
      if (lastMove === k) {
        cell.classList.add("last-move");
      }

      const piece = pieces.get(k);
      if (piece) {
        const token = document.createElement("span");
        token.className = `stone ${piece.side > 0 ? "black" : "white"}`;
        cell.appendChild(token);
      }

      if (!snapshot.game_over) {
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
}

bootstrap().catch((err) => {
  statusEl.textContent = `初始化失败: ${err}`;
});
