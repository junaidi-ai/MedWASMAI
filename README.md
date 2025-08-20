# MedWASMAI 🧬🩺
MedWASMAI is an open-source framework for deploying high-performance, secure Edge AI models in healthcare and Internet of Medical Things (IoMT) using WebAssembly (WASM) and Rust. 

Our mission is to enable real-time, privacy-preserving machine learning (ML) inference on resource-constrained devices like wearables, medical sensors, and remote monitors, empowering applications such as vital sign anomaly detection, federated learning for patient data, and offline diagnostics in underserved regions.

Built with Rust for safety and performance, MedWASMAI leverages WASM's portability and sandboxed execution to deliver near-native ML inference on edge devices while adhering to healthcare standards like HIPAA and GDPR. Whether you're analyzing ECG data or enabling AI-driven prosthetics, MedWASMAI provides a lightweight, scalable solution for IoMT innovation.

## Features ✨

- 🧬 Edge AI for IoMT: Run ML models (e.g., TensorFlow Lite, ONNX) directly on edge devices using WASM, minimizing latency and cloud dependency.
- 🦀 Rust-Powered: Leverage Rust's memory safety and performance for robust WASM modules tailored to healthcare use cases.
- 🔒 Privacy-First: Support for differential privacy and secure execution to protect sensitive medical data.
- 🌍 Cross-Platform: Deploy on browsers, IoT devices (e.g., Raspberry Pi, ESP32), or servers with WasmEdge/Wasmtime runtimes.
- 🩺 Healthcare Focus: Designed for applications like real-time vital monitoring, medical imaging, and federated learning in IoMT.

## Getting Started 🚀

### Prerequisites 📦

- Rust: Install the latest stable version via rustup.
- WASM Toolchain: Install wasm-pack for compiling Rust to WASM:

```bash
cargo install wasm-pack
```

- WasmEdge: Install the WasmEdge runtime for edge execution:

```bash
curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash
```

- Node.js (optional): For browser-based demos, install Node.js 16+.
- Python (optional): For ML model preparation (e.g., TensorFlow Lite, ONNX).
- Hardware: Raspberry Pi or similar for IoMT testing (optional).

### Installation 🛠️

Clone the repository:

```bash
git clone https://github.com/Junaidi-ai/MedWASMAI.git
cd MedWASMAI
```

### Build the Rust WASM module 🔧

```bash
wasm-pack build --target web
```

### Run a demo (e.g., heart rate anomaly detection) ▶️

```bash
wasmedge --dir .:. pkg/medwasmai.wasm
```

### Browser demo 🌐

```bash
npm install
npm start
```

Open http://localhost:8080 in a browser to see the demo.

## Example: Heart Rate Anomaly Detection ❤️

This example compiles a simple ML model to WASM for detecting anomalies in heart rate data on an IoMT device.

1. Train a lightweight model (e.g., using TensorFlow Lite) on a dataset like PhysioNet ECG.
2. Convert the model to WASM using wasm-pack and integrate with Rust.
3. Run inference on an edge device:

```rust
use wasmedge_tensorflow_lite::*;

fn main() {
    let model = include_bytes!("heart_rate_model.tflite");
    let input = vec![/* heart rate data */];
    let output = run_inference(model, &input);
    println!("Anomaly detected: {:?}", output);
}
```

See the `examples/` directory for complete code and setup.

## Contributing 🤝

We welcome contributions from the community! To get started:

- Fork and clone the repo.
- Pick an issue: Check the Issues tab for tasks like adding new models, optimizing WASM performance, or creating IoMT demos.
- Submit a PR: Follow the contribution guidelines and submit a pull request with clear descriptions.
- Join the discussion: Share ideas on GitHub Discussions or tag `@JunaidiAI` on X with `#MedWASMAI`.

We especially need help with:

- Healthcare-specific ML models (e.g., biosignal processing)
- WASM optimizations for low-power IoMT devices
- Privacy-preserving techniques (e.g., homomorphic encryption)
- Documentation and tutorials for IoMT developers

## Roadmap 🗺️

- Initial WASM module for ML inference in Rust
- Support for WebGPU acceleration on edge devices
- Federated learning integration for IoMT networks
- HIPAA/GDPR-compliant data pipelines
- Demos for specific use cases (e.g., glucose monitoring, medical imaging)

## License 📄

This project is licensed under the MIT License. See `LICENSE` for details.

## Related Docs 📚

- Rust install (rustup): https://rustup.rs/
- wasm-pack: https://rustwasm.github.io/wasm-pack/
- WasmEdge install guide: https://wasmedge.org/book/en/quick_start/install.html
- Node.js downloads: https://nodejs.org/en/download
- TensorFlow Lite model conversion: https://www.tensorflow.org/lite/convert
- ONNX docs: https://onnx.ai/
- PhysioNet ECG databases: https://physionet.org/
- License (MIT): [LICENSE](./LICENSE)
- Contribution Guidelines: [CONTRIBUTING.md](./CONTRIBUTING.md)
- Examples: [examples/](./examples/)

## Contact 📫

- 🏢 Organization: [Junaidi AI](https://junaidi-ai.id) 
- 🐙 GitHub: https://github.com/SHA888
- 🐦 X (Twitter): https://x.com/ks_sha888
- ✉️ Email: kresnasucandra@gmail.com

---

_Join us in revolutionizing Edge AI for healthcare with WebAssembly! 🚀_