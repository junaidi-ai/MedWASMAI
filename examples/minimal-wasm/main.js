import init, { detect_anomaly } from "./pkg/minimal_wasm.js";

const statusEl = document.getElementById("status");
const runBtn = document.getElementById("run");
const outEl = document.getElementById("out");
const valEl = document.getElementById("val");

async function main() {
  try {
    statusEl.textContent = "Initializing WASM...";
    await init();
    statusEl.textContent = "Ready";
    statusEl.className = "badge ok";
    runBtn.disabled = false;
  } catch (err) {
    console.error(err);
    statusEl.textContent = "Failed to init WASM";
    statusEl.className = "badge warn";
  }
}

runBtn.addEventListener("click", () => {
  const v = parseFloat(valEl.value);
  if (Number.isNaN(v)) {
    outEl.textContent = "Please enter a number";
    outEl.className = "warn";
    return;
  }
  const res = detect_anomaly(v);
  outEl.textContent = res ? `Anomaly detected at ${v}` : `No anomaly at ${v}`;
  outEl.className = res ? "warn" : "ok";
});

main();
