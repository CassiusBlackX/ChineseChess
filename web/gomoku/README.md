# 五子棋 Web 演示

## 构建 WASM

在项目根目录执行：

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
wasm-pack build crates/gomoku --target web --out-dir pkg/gomoku
```

## 运行

在项目根目录启动静态文件服务，例如：

```bash
python -m http.server 8000
```

浏览器打开：<http://127.0.0.1:8000/web/gomoku/>
