use wasm_bindgen::prelude::*;

/// Minimal example: a tiny function that flags a simple threshold as an "anomaly".
/// Build with: `wasm-pack build --target web`
#[wasm_bindgen]
pub fn detect_anomaly(value: f32) -> bool {
    value > 0.5
}
