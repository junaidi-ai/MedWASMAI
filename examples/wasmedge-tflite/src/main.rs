 use std::path::PathBuf;
 use std::time::Instant;
 use std::fs;

use anyhow::{bail, Context, Result};
use clap::Parser;
 use log::{info, warn};

/// Minimal WasmEdge + TFLite example scaffold.
/// By default this performs a mock inference so it can run anywhere.
/// Enable the `real-inference` feature and fill in the real implementation once
/// your environment has WasmEdge + TFLite plugin set up.
#[derive(Parser, Debug)]
#[command(name = "wasmedge-tflite-example")] 
#[command(about = "Run a mock or real (feature-gated) TFLite inference flow", long_about = None)]
struct Args {
    /// Path to a .tflite model (optional).
    #[arg(short, long)]
    model: Option<PathBuf>,

    /// Comma-separated list of float inputs, e.g. "0.1,0.2,0.3".
    #[arg(short, long, default_value = "0.1,0.2,0.3,0.4")]
    input: String,

    /// Additional plugin directory to search (WasmEdge plugins).
    #[arg(long, env = "WASMEDGE_PLUGIN_PATH")] 
    plugin_dir: Option<PathBuf>,

    /// Use mock inference even if real inference is compiled in.
    #[arg(long)]
    force_mock: bool,
}

fn parse_inputs(input: &str) -> Result<Vec<f32>> {
    let mut values = Vec::new();
    for (idx, tok) in input.split(',').enumerate() {
        let v: f32 = tok.trim().parse().with_context(|| format!("invalid float at position {}: '{}'", idx, tok))?;
        values.push(v);
    }
    if values.is_empty() {
        bail!("no inputs provided");
    }
    Ok(values)
}

fn ensure_model_exists(path: &PathBuf) -> Result<()> {
    if !path.exists() {
        bail!("model not found: {}", path.display());
    }
    if path.extension().and_then(|s| s.to_str()) != Some("tflite") {
        warn!("model does not have .tflite extension: {}", path.display());
    }
    Ok(())
}

fn run_mock_inference(input: &[f32]) -> Vec<f32> {
    // Simple transform to simulate an inference result.
    input.iter().map(|v| (v * 2.0).tanh()).collect()
}

#[cfg(feature = "real-inference")]
fn run_real_inference(model_path: &PathBuf, input: &[f32], plugin_dir: Option<PathBuf>) -> Result<Vec<f32>> {
    // Minimal setup using wasmedge-sdk to demonstrate VM configuration.
    use wasmedge_sdk::{config::Config, Vm};

    // Ensure model exists and load bytes (already checked by caller, but double-safe)
    ensure_model_exists(model_path)?;
    let _model_bytes = fs::read(model_path)
        .with_context(|| format!("failed to read model bytes: {}", model_path.display()))?;

    // Configure WasmEdge VM and plugins
    let mut config = Config::create().context("failed to create WasmEdge Config")?;
    if let Some(dir) = plugin_dir {
        config
            .add_plugin_dir(dir.as_path())
            .with_context(|| format!("failed to add plugin dir: {}", dir.display()))?;
    }
    let _vm = Vm::create(Some(config), None).context("failed to create WasmEdge VM")?;

    // NOTE: A complete flow would:
    // - Provide a guest wasm using wasi-nn or wasmedge_tensorflow_interface
    // - Register and run that guest with the VM
    // For now, return a clear error guiding next steps.
    bail!(
        "real inference path initialized VM, but no guest wasm is provided. \
Please supply a wasi-nn guest and invoke it via WasmEdge. See README for guidance.\n\
Inputs seen: {} values; Model: {}",
        input.len(),
        model_path.display()
    )
}

#[cfg(not(feature = "real-inference"))]
fn run_real_inference(_model_path: &PathBuf, _input: &[f32], _plugin_dir: Option<PathBuf>) -> Result<Vec<f32>> {
    anyhow::bail!("binary was built without 'real-inference' feature. Rebuild with --features real-inference")
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();
    info!("args: {:?}", args);

    // Parse inputs
    let input_values = parse_inputs(&args.input)?;

    // If a model is specified, ensure it exists
    if let Some(ref model) = args.model {
        ensure_model_exists(model)?;
        info!("using model: {}", model.display());
        // Demonstrate model loading
        let bytes = fs::read(model).with_context(|| format!("failed to read model bytes: {}", model.display()))?;
        info!("model bytes loaded: {} B", bytes.len());
    } else {
        warn!("no model provided; mock path will be used if running mock inference");
    }

    // Decide which path to take and measure latency
    let start = Instant::now();
    let (path_used, outputs): (&str, Vec<f32>) = if args.force_mock {
        info!("running mock inference (forced)");
        ("mock", run_mock_inference(&input_values))
    } else if let Some(ref model) = args.model {
        match run_real_inference(model, &input_values, args.plugin_dir.clone()) {
            Ok(out) => ("real", out),
            Err(e) => {
                warn!("real inference unavailable: {}", e);
                info!("falling back to mock inference");
                ("mock-fallback", run_mock_inference(&input_values))
            }
        }
    } else {
        info!("running mock inference (no model provided)");
        ("mock", run_mock_inference(&input_values))
    };
    let elapsed = start.elapsed();

    info!("inference path: {} | latency: {:?}", path_used, elapsed);
    println!("path={} result={:?}", path_used, outputs);
    Ok(())
}
