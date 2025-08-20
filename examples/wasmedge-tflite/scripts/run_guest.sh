#!/usr/bin/env bash
set -euo pipefail

# Run the wasi-nn guest module with WasmEdge.
# Usage:
#   ./scripts/run_guest.sh <model_path.tflite> "0.1,0.2,0.3" [WASMEDGE_PLUGIN_PATH]
# Example:
#   ./scripts/run_guest.sh models/simple_model.tflite "0.1,0.2,0.3,0.4" /usr/local/lib/wasmedge

if [ $# -lt 2 ]; then
  echo "usage: $0 <model_path.tflite> <comma_separated_floats> [WASMEDGE_PLUGIN_PATH]" 1>&2
  exit 1
fi

MODEL="$1"
INPUT="$2"
PLUGIN_PATH="${3:-}"

here="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$here/.." && pwd)"
wasm="$root/guest/target/wasm32-wasi/release/guest-wasi-nn.wasm"

if [ ! -f "$wasm" ]; then
  echo "error: wasm not found at $wasm" 1>&2
  echo "hint: build it with: (cd guest && cargo build --release --target wasm32-wasi)" 1>&2
  exit 1
fi

if [ ! -f "$MODEL" ]; then
  echo "error: model not found at $MODEL" 1>&2
  exit 1
fi

if [ -n "$PLUGIN_PATH" ]; then
  export WASMEDGE_PLUGIN_PATH="$PLUGIN_PATH"
fi

# Preopen current directory so guest can read model path relative to project root
exec wasmedge --dir .:. "$wasm" --model "$MODEL" --input "$INPUT"
