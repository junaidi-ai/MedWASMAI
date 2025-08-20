# Examples

![Demo](./minimal-wasm/public/Minimal_Wasm_Demo.png)

This directory contains example projects for MedWASMAI.

- `minimal-wasm/`: A minimal Rust + wasm-bindgen project you can build with `wasm-pack`.
 - `wasmedge-tflite/`: A Rust CLI scaffold to demonstrate running TensorFlow Lite inference with WasmEdge (mock by default; real path is feature-gated).

## Build minimal-wasm

```bash
cd examples/minimal-wasm
wasm-pack build --target web
```

The build output will be in `pkg/`.

## Run minimal-wasm in the browser

You need to serve the folder over HTTP (not file://) due to module loading and CORS.

Option A: Python (built-in)

```bash
cd examples/minimal-wasm
python3 -m http.server 8080
```

Then open:

```
http://localhost:8080/
```

Option B: Node.js (serve)

```bash
npm -g install serve
cd examples/minimal-wasm
serve -l 8080
```

Open `index.html` and press "Run" after the status shows "Ready". Adjust the input value to test `detect_anomaly(value: f32)`.

## Build wasmedge-tflite

```bash
cd examples/wasmedge-tflite
cargo build
```

Run mock inference with logs:

```bash
RUST_LOG=info ./target/debug/wasmedge-tflite-example --force-mock --input "0.1,0.2,0.3,0.4"
```

See `examples/wasmedge-tflite/README.md` for enabling the real path and using a `.tflite` model.
