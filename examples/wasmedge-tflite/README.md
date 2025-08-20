# WasmEdge + TFLite Example (Scaffold)

This example demonstrates a minimal structure for running TensorFlow Lite inference with WasmEdge.
By default, it runs a mock inference so you can build and run it without setting up the TFLite plugin.

Once your environment has WasmEdge built with the TFLite plugin and a proper guest module (e.g., using wasi-nn),
you can switch to the real path (feature-gated in this scaffold) and wire in your flow.

## Layout

- `src/main.rs` — CLI app with mock inference and a placeholder for real inference
- `models/` — put your `.tflite` model files here (e.g., `models/simple_model.tflite`)
- `scripts/download_model.sh` — helper script (placeholder) to fetch a test model

## Requirements

- Rust stable (via rustup)
- WasmEdge runtime installed
- Optional: WasmEdge TFLite plugin available and configured

## Build

```bash
cd examples/wasmedge-tflite
cargo build
```

## Run (mock inference)

```bash
./target/debug/wasmedge-tflite-example --input "0.1,0.2,0.3,0.4" --force-mock
```

You should see output like:

```
Inference result: [0.19737533, 0.37994897, 0.5370496, 0.66403675]
```

## Run (attempt real inference)

1) Place a TFLite model:

```bash
mkdir -p models
# Put your file at models/simple_model.tflite
```

2) Run with a model path. If the real path is not implemented or plugin is missing, the app will fall back to mock.

```bash
./target/debug/wasmedge-tflite-example --model models/simple_model.tflite --input "0.1,0.2,0.3,0.4"
```

3) If you have the WasmEdge TFLite plugin installed in a non-default folder, set the plugin path:

```bash
export WASMEDGE_PLUGIN_PATH=/path/to/wasmedge/plugins
./target/debug/wasmedge-tflite-example --model models/simple_model.tflite
```

4) Enable the feature gate (scaffold) to switch from mock to a real path that you can implement:

```bash
cargo run --features real-inference -- --model models/simple_model.tflite
```

Note: In this scaffold, the real path returns an error until you integrate your own guest module flow.

## Logging

Set `RUST_LOG` for more logs:

```bash
RUST_LOG=info ./target/debug/wasmedge-tflite-example --force-mock
```

## Measuring latency

The app logs basic latency around the inference call. For more precise benchmarking, consider `hyperfine` or `criterion`.

## Troubleshooting

- Missing model: ensure the path exists, e.g., `models/simple_model.tflite`
- Plugin issues: set `WASMEDGE_PLUGIN_PATH` to the directory containing WasmEdge plugins
- Real inference not implemented: this scaffold ships with a placeholder; wire in your specific setup using wasi-nn/guest wasm

## Test Strategy

- Verify build: `cd examples/wasmedge-tflite && cargo build`
- Test with a simple TFLite model:
  - Use `scripts/download_model.sh <URL> simple_model.tflite` (run `chmod +x scripts/download_model.sh` first if needed)
  - Run: `./target/debug/wasmedge-tflite-example --model models/simple_model.tflite --input "0.1,0.2,0.3,0.4"`
- Measure latency: observe `RUST_LOG=info` output for inference latency; for more precise benchmark use tools like `hyperfine`.
- Edge device (e.g., Raspberry Pi):
  - Install Rust and WasmEdge on the device
  - Build natively or cross-compile; run the same commands. Set `WASMEDGE_PLUGIN_PATH` if plugins are in a non-default directory.
- Error handling checks:
  - Missing model path should produce a clear error
  - Invalid inputs (e.g., non-floats in `--input`) should surface informative parse errors

## Guest (wasi-nn) path

If you want a fully working TensorFlow Lite inference flow under WasmEdge today, build and run the provided wasi-nn guest module. It uses the WasmEdge WASI-NN TensorFlow Lite backend.

Prerequisites:

- WasmEdge installed with the WASI-NN TensorFlow Lite backend
- Rust target for WASI: `rustup target add wasm32-wasi`

Build the guest:

```bash
cd examples/wasmedge-tflite/guest
cargo build --release --target wasm32-wasi
```

Download or place a `.tflite` model at `../models/simple_model.tflite`.

Run with WasmEdge:

```bash
cd examples/wasmedge-tflite
export WASMEDGE_PLUGIN_PATH=/path/to/wasmedge/plugins  # if not in default location
wasmedge --dir .:. guest/target/wasm32-wasi/release/guest-wasi-nn.wasm \
  --model models/simple_model.tflite \
  --input "0.1,0.2,0.3,0.4"
```

The guest will print the output tensor as CSV to stdout.
