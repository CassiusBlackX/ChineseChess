# 棋类游戏引擎（Rust）

通用棋类引擎框架，当前内置**中国象棋**与**五子棋**，支持桌面 GUI、终端 TUI 和浏览器 WASM。

## 项目结构

```
crates/
  board_engine/   通用网格原语（Position、Grid）
  game_view/      UI 契约（SnapshotDto、GameViewAdapter）
  xiangqi/        中国象棋规则
  gomoku/         五子棋规则（15×15，黑先，五连胜）
  game_app/       统一启动器（GUI / TUI）
web/
  xiangqi/        象棋 WASM 网页
  gomoku/         五子棋 WASM 网页
```

## 环境

- 原生端：Windows / macOS / Linux
- Web 端需要：`wasm32-unknown-unknown` 目标、`wasm-pack`

## 运行

### 桌面 / 终端

```bash
cargo run -p game_app -- xiangqi gui   # 中国象棋 · 原生窗口
cargo run -p game_app -- xiangqi tui   # 中国象棋 · 终端
cargo run -p game_app -- gomoku gui    # 五子棋 · 原生窗口
cargo run -p game_app -- gomoku tui    # 五子棋 · 终端
cargo run -p game_app                  # 交互式选择游戏与模式
```

### 浏览器

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack

wasm-pack build crates/xiangqi --target web --out-dir pkg/xiangqi
wasm-pack build crates/gomoku --target web --out-dir pkg/gomoku
```

在项目根目录启动静态文件服务，例如：

```bash
python -m http.server 8000
```

- 中国象棋：<http://127.0.0.1:8000/web/xiangqi/>
- 五子棋：<http://127.0.0.1:8000/web/gomoku/>

## 测试

```bash
cargo test --workspace
```

## 扩展新游戏

1. 在 `crates/` 下新建游戏 crate，依赖 `board_engine` 与 `game_view`
2. 实现 `GameViewAdapter` trait
3. 在 `game_app/src/launcher.rs` 注册游戏
4. 可选：添加 `wasm_api.rs` 与 `web/<game>/` 前端
