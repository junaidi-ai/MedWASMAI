#!/usr/bin/env bash
set -euo pipefail

# Simple helper to place a .onnx model into ./models.
# Usage:
#   ./scripts/download_model.sh <URL> [output_name.onnx]
# Example:
#   ./scripts/download_model.sh "https://example.com/path/to/model.onnx" simple_model.onnx
#
# Note: We do not bundle any model in this repo to keep size and licensing simple.

here="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$here/.." && pwd)"
models_dir="$root/models"
mkdir -p "$models_dir"

if ! command -v curl >/dev/null 2>&1; then
  echo "error: curl is required" 1>&2
  exit 1
fi

if [[ $# -lt 1 ]]; then
  cat 1>&2 <<'USAGE'
error: missing URL

Usage:
  ./scripts/download_model.sh <URL> [output_name.onnx]

Notes:
- This repo does not ship a model by default. Provide a valid public URL to a .onnx file.
- The file will be saved under examples/wasmedge-onnx/models.
USAGE
  exit 2
fi

url="$1"
name="${2:-downloaded_model.onnx}"

out="$models_dir/$name"

echo "Downloading: $url" >&2
curl -L "$url" -o "$out"

if [[ ! -s "$out" ]]; then
  echo "error: downloaded file is empty: $out" 1>&2
  exit 3
fi

echo "Saved: $out" >&2
