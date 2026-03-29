# Chinese Chess WASM Demo

## Build WASM

1. Install target:

   rustup target add wasm32-unknown-unknown

2. Install wasm-pack:

   cargo install wasm-pack

3. Build pkg directory from Rust logic layer:

   wasm-pack build --target web --out-dir pkg

## Run Web Demo

Open a static file server at project root, for example:

python -m http.server 8000

Then open:

http://127.0.0.1:8000/web/

## Architecture

- Rust rules and state are in src/ and exported via src/wasm_api.rs.
- Rendering and input handling are in web/app.js.
- Frontend only consumes snapshots and move APIs from WASM, so you can replace rendering with GPUI or Tauri later.
