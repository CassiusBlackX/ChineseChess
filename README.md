# 中国象棋（Rust）

用 Rust 实现的中国象棋规则与对局逻辑，带桌面 GUI、终端 TUI 和浏览器 WASM 演示。

## 环境

- 原生端：Windows / macOS / Linux
- Web 端需要：`wasm32-unknown-unknown` 目标、`wasm-pack`

## 运行

### 桌面 / 终端

```bash
cargo run          # 启动时选择 GUI 或 TUI
cargo run -- gui   # 原生窗口（eframe / egui）
cargo run -- tui   # 终端界面（ratatui）
```

### 浏览器

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
wasm-pack build --target web --out-dir pkg
```

在项目根目录起一个静态文件服务，例如：

```bash
python -m http.server 8000
```

浏览器打开：<http://127.0.0.1:8000/web/>

## 测试

```bash
cargo test
```

## 目录

```
src/           棋盘、棋子走法、对局状态、视图适配
src/ui/        桌面 GUI、终端 TUI
src/wasm_api.rs  WASM 导出（仅 wasm32）
web/           网页棋盘与交互
```

规则与状态在 `src/`；各端 UI 只消费快照（`SnapshotDto`）和点击/走子接口。