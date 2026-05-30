import init, { WasmGame } from "../../pkg/xiangqi/xiangqi.js";

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

function moveSet(moves) {
  const set = new Set();
  for (const m of moves) {
    set.add(keyOf(m.x, m.y));
  }
  return set;
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
  statusEl.textContent = `${snapshot.message} | 当前回合: ${turnText}${gameStateText}`;

  const pieces = pieceMap(snapshot.pieces);
  const moves = moveSet(snapshot.legal_moves);
  const selected = snapshot.selected ? keyOf(snapshot.selected.x, snapshot.selected.y) : null;

  boardEl.style.gridTemplateColumns = `repeat(${width}, 1fr)`;
  boardEl.style.gridTemplateRows = `repeat(${height}, 1fr)`;
  boardEl.style.aspectRatio = `${width} / ${height}`;

  boardEl.innerHTML = "";
  for (let y = height - 1; y >= 0; y -= 1) {
    for (let x = 0; x < width; x += 1) {
      const k = keyOf(x, y);
      const cell = document.createElement("button");
      cell.className = "cell";
      cell.disabled = Boolean(snapshot.game_over);
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
