# WasmEdge + ONNX Example (Scaffold)

This example demonstrates a minimal structure for running ONNX model inference with WasmEdge.
By default, it runs a mock inference so you can build and run it without setting up the ONNX plugin.

Once your environment has WasmEdge built with the ONNX runtime plugin and a proper guest module (e.g., using wasi-nn with an ONNX backend),
you can switch to the real path (feature-gated in this scaffold) and wire in your flow.

## Layout

- `src/main.rs` — CLI app with mock inference and a placeholder for real inference
- `models/` — put your `.onnx` model files here (e.g., `models/simple_model.onnx`)
- `scripts/download_model.sh` — helper script (placeholder) to fetch a test model

## Requirements

- Rust stable (via rustup)
- WasmEdge runtime installed
- Optional: WasmEdge ONNX plugin available and configured

## Build

```bash
cd examples/wasmedge-onnx
cargo build
```

### Build the ONNX guest (wasi-nn)

This repository ships a WASI guest that can run with a wasi-nn ONNX backend when available.

```bash
cd examples/wasmedge-onnx/guest
cargo build --release --target wasm32-wasi
```

Notes:

- By default, the guest uses a safe fallback (no wasi-nn) and simply transforms inputs so the pipeline is runnable everywhere.
- To enable real ONNX execution inside the guest, build with the feature flag (requires wasi-nn and the WasmEdge ONNX backend):

```bash
cargo build --release --target wasm32-wasi --features onnx-real
```

## Run (mock inference)

```bash
./target/debug/wasmedge-onnx-example --input "0.1,0.2,0.3,0.4" --force-mock
```

You should see output like:

```
Inference result: [ ... ]
```

## Run (attempt real inference)

1) Place an ONNX model:

```bash
mkdir -p models
# Put your file at models/simple_model.onnx
```

2) Run with a model path. If the real path is not implemented or plugin is missing, the app will fall back to mock.

```bash
./target/debug/wasmedge-onnx-example --model models/simple_model.onnx --input "0.1,0.2,0.3,0.4"
```

3) If you have the WasmEdge ONNX plugin installed in a non-default folder, set the plugin path:

```bash
export WASMEDGE_PLUGIN_PATH=/path/to/wasmedge/plugins
./target/debug/wasmedge-onnx-example --model models/simple_model.onnx
```

4) Enable the feature gate (scaffold) to switch from mock to a real path that you can implement:

```bash
cargo run --features real-inference -- --model models/simple_model.onnx
```

Note: In this scaffold, the real path returns an error until you integrate your own guest module flow.

### Run with the ONNX guest via WasmEdge (host real path)

The host can execute the WASI guest using the WasmEdge CLI. Build the guest first (see above), then run the host with `real-inference` enabled:

```bash
cd examples/wasmedge-onnx

# Build host with real path
cargo build --features real-inference

# If your WasmEdge plugins are in a non-default directory, export the path
# export WASMEDGE_PLUGIN_PATH=/path/to/wasmedge/plugins

# Place or download a model (see script below), then run:
RUST_LOG=info ./target/debug/wasmedge-onnx-example \
  --model models/simple_model.onnx \
  --input "0.1,0.2,0.3,0.4"
```

The host will invoke:

```
wasmedge --dir .:. guest/target/wasm32-wasi/{release|debug}/onnx-guest.wasm --model <path> --input <csv>
```

## Logging

Set `RUST_LOG` for more logs:

```bash
RUST_LOG=info ./target/debug/wasmedge-onnx-example --force-mock
```

## Measuring latency

The app logs basic latency around the inference call. For more precise benchmarking, consider `hyperfine` or `criterion`.

## Troubleshooting

- Missing model: ensure the path exists, e.g., `models/simple_model.onnx`
- Plugin issues: set `WASMEDGE_PLUGIN_PATH` to the directory containing WasmEdge plugins
- Real inference not implemented: this scaffold ships with a placeholder; wire in your specific setup using wasi-nn/guest wasm

## Test Strategy

- Verify build: `cd examples/wasmedge-onnx && cargo build`
- Test with a simple ONNX model:
  - Use `scripts/download_model.sh <URL> simple_model.onnx` (run `chmod +x scripts/download_model.sh` first if needed)
  - Run: `./target/debug/wasmedge-onnx-example --model models/simple_model.onnx --input "0.1,0.2,0.3,0.4"`
- Measure latency: observe `RUST_LOG=info` output for inference latency; for more precise benchmark use tools like `hyperfine`.
- Edge device (e.g., Raspberry Pi):
  - Install Rust and WasmEdge on the device
  - Build natively or cross-compile; run the same commands. Set `WASMEDGE_PLUGIN_PATH` if plugins are in a non-default directory.
- Error handling checks:
  - Missing model path should produce a clear error
  - Invalid inputs (e.g., non-floats in `--input`) should surface informative parse errors

## Supported operators and limitations

Supported ONNX operators and performance characteristics depend on the ONNX backend configured for WasmEdge (e.g., WasmEdge ONNX Runtime plugin or wasi-nn backends).
Start with small models and consult your backend's documentation for operator coverage, input shapes, and precision constraints.

## Benchmarks

Record your results here after running end-to-end. Example template:

```
Device: x86-64 (12th Gen Intel i7-xxxx), Linux 6.x
WasmEdge: X.Y.Z, ONNX backend: <name/version>
Model: simple_model.onnx (input shape: [N])

Mode           Mean (ms)  p95 (ms)  Notes
-------------  ---------  --------  -------------------------------
Mock (host)       0.20       0.30   args: --force-mock
Real (guest)      3.10       4.00   host -> wasmedge -> guest
```

For Raspberry Pi, repeat the table with Pi hardware and OS details, and note any plugin-path differences.
