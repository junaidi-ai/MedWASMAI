#!/usr/bin/env bash
# Verify the Minimal WASM browser demo end-to-end
# - Checks dependencies (Rust, wasm-pack, Node, Python)
# - Builds the WASM module
# - Starts a local HTTP server
# - Attempts to open the browser (best effort)
# - Prints clear success/failure status

set -euo pipefail

PORT="${PORT:-8080}"
DEMO_DIR="examples/minimal-wasm"
URL="http://localhost:${PORT}"

# Colors
if [[ -t 1 ]]; then
  GREEN='\033[0;32m'
  RED='\033[0;31m'
  YELLOW='\033[0;33m'
  BLUE='\033[0;34m'
  NC='\033[0m'
else
  GREEN='' ; RED='' ; YELLOW='' ; BLUE='' ; NC=''
fi

info()    { echo -e "${BLUE}[INFO]${NC}  $*"; }
success() { echo -e "${GREEN}[OK]${NC}    $*"; }
warn()    { echo -e "${YELLOW}[WARN]${NC}  $*"; }
error()   { echo -e "${RED}[ERROR]${NC} $*"; }

check_dep() {
  local dep="$1"
  local hint="$2"
  if ! command -v "$dep" >/dev/null 2>&1; then
    error "Missing dependency: $dep"
    [[ -n "$hint" ]] && echo "  -> $hint"
    return 1
  fi
}

cleanup() {
  if [[ -n "${SERVER_PID:-}" ]] && ps -p "$SERVER_PID" >/dev/null 2>&1; then
    info "Stopping server (pid=$SERVER_PID)"
    kill "$SERVER_PID" >/dev/null 2>&1 || true
    wait "$SERVER_PID" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT
trap 'cleanup; exit 130' INT TERM

info "Checking dependencies..."
missing=0
check_dep rustc     "Install Rust via rustup: https://rustup.rs" || missing=1
check_dep wasm-pack "Install: cargo install wasm-pack" || missing=1
check_dep node      "Install Node.js (v20+ recommended): https://nodejs.org" || missing=1
check_dep python3   "Install Python 3 for the local HTTP server" || missing=1

if [[ "$missing" -ne 0 ]]; then
  error "One or more required dependencies are missing. See hints above."
  exit 1
fi
success "All dependencies found."

info "Building WASM module with wasm-pack..."
(
  cd "$DEMO_DIR"
  wasm-pack build --target web
)
success "WASM build complete."

info "Starting local HTTP server on port $PORT..."
(
  cd "$DEMO_DIR"
  # Suppress server logs; comment out redirection to see verbose output
  python3 -m http.server "$PORT" >/dev/null 2>&1 &
  SERVER_PID=$!
  echo "$SERVER_PID" > ../.server_pid.tmp
)
# Read SERVER_PID back in the parent shell
if [[ -f "examples/.server_pid.tmp" ]]; then
  SERVER_PID=$(cat examples/.server_pid.tmp || true)
  rm -f examples/.server_pid.tmp || true
else
  # Fallback if temp file location changes
  SERVER_PID=${SERVER_PID:-}
fi

# Wait for server to become responsive
info "Waiting for server to be ready at $URL ..."
ready=0
if command -v curl >/dev/null 2>&1; then
  for i in {1..40}; do
    if curl -fsS "$URL" >/dev/null 2>&1; then
      ready=1
      break
    fi
    sleep 0.25
  done
else
  warn "curl not found; sleeping briefly instead of probing"
  sleep 2
  ready=1
fi

if [[ "$ready" -ne 1 ]]; then
  error "Server did not become ready at $URL"
  exit 1
fi
success "Server is running at $URL"

open_url() {
  local url="$1"
  # WSL-specific opener
  if command -v wslview >/dev/null 2>&1; then wslview "$url" && return 0; fi
  # Linux
  if command -v xdg-open >/dev/null 2>&1; then xdg-open "$url" >/dev/null 2>&1 && return 0; fi
  # macOS
  if command -v open >/dev/null 2>&1; then open "$url" >/dev/null 2>&1 && return 0; fi
  # Git Bash / Cygwin
  if command -v cygstart >/dev/null 2>&1; then cygstart "$url" >/dev/null 2>&1 && return 0; fi
  # Windows PowerShell (rare in WSL):
  if command -v powershell.exe >/dev/null 2>&1; then powershell.exe start "$url" >/dev/null 2>&1 && return 0; fi
  return 1
}

if [[ -z "${NO_OPEN:-}" ]]; then
  if open_url "$URL"; then
    info "Opened browser to $URL"
  else
    warn "Could not auto-open a browser. Please navigate to: $URL"
  fi
else
  warn "NO_OPEN is set; not attempting to open the browser. URL: $URL"
fi

cat <<EOF
${GREEN}
Verification running.
- URL: $URL
- Directory: $DEMO_DIR

Press Ctrl+C to stop the server.
${NC}
EOF

# Keep the script attached to the server process
if [[ -n "${SERVER_PID:-}" ]]; then
  wait "$SERVER_PID"
else
  # Fallback: sleep to keep context if PID detection failed
  tail -f /dev/null
fi
