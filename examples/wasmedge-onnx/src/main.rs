use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use std::process::Command;

use anyhow::{bail, Context, Result};
use clap::Parser;
use log::{info, warn};

/// Minimal WasmEdge + ONNX example scaffold.
/// By default this performs a mock inference so it can run anywhere.
/// Enable the `real-inference` feature and fill in the real implementation once
/// your environment has WasmEdge + ONNX plugin and a guest wasm (e.g., using wasi-nn).
#[derive(Parser, Debug)]
#[command(name = "wasmedge-onnx-example")]
#[command(about = "Run a mock or real (feature-gated) ONNX inference flow", long_about = None)]
struct Args {
    /// Path to an .onnx model (optional).
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
        let v: f32 = tok
            .trim()
            .parse()
            .with_context(|| format!("invalid float at position {}: '{}'", idx, tok))?;
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
    if path.extension().and_then(|s| s.to_str()) != Some("onnx") {
        warn!("model does not have .onnx extension: {}", path.display());
    }
    Ok(())
}

fn run_mock_inference(input: &[f32]) -> Vec<f32> {
    // Simple transform to simulate an inference result.
    input.iter().map(|v| (v * 1.5).tanh()).collect()
}

#[cfg(feature = "real-inference")]
fn run_real_inference(
    model_path: &PathBuf,
    input: &[f32],
    plugin_dir: Option<PathBuf>,
) -> Result<Vec<f32>> {
    // Ensure model exists and is readable
    ensure_model_exists(model_path)?;
    let _ = fs::metadata(model_path)
        .with_context(|| format!("failed to stat model: {}", model_path.display()))?;

    // Locate guest wasm (prefer release, then debug)
    let mut guest_wasm = PathBuf::from("guest/target/wasm32-wasi/release/onnx-guest.wasm");
    if !guest_wasm.exists() {
        let debug_path = PathBuf::from("guest/target/wasm32-wasi/debug/onnx-guest.wasm");
        if debug_path.exists() {
            guest_wasm = debug_path;
        } else {
            bail!(
                "guest wasm not found at {} or {}. Build it with: \n  cd examples/wasmedge-onnx/guest && cargo build --release --target wasm32-wasi",
                guest_wasm.display(),
                debug_path.display()
            );
        }
    }

    // Build input CSV
    let csv = input.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",");

    // Prepare WasmEdge CLI command
    let mut cmd = Command::new("wasmedge");
    cmd.arg("--dir").arg(".:.");
    if let Some(dir) = plugin_dir {
        cmd.env("WASMEDGE_PLUGIN_PATH", dir);
    }
    cmd.arg(&guest_wasm)
        .arg("--model").arg(model_path)
        .arg("--input").arg(csv);

    let out = cmd.output().context("failed to execute wasmedge CLI; ensure WasmEdge is installed and on PATH")?;
    if !out.status.success() {
        bail!(
            "guest execution failed (status={}):\nstdout:\n{}\nstderr:\n{}",
            out.status,
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
    }

    let stdout = String::from_utf8(out.stdout).context("guest stdout was not valid UTF-8")?;
    let line = stdout.trim();
    if line.is_empty() {
        bail!("guest did not print any output");
    }
    let mut outputs = Vec::new();
    for (idx, tok) in line.split(',').enumerate() {
        let v: f32 = tok.trim().parse().with_context(|| format!("invalid float at position {}: '{}'", idx, tok))?;
        outputs.push(v);
    }
    Ok(outputs)
}

#[cfg(not(feature = "real-inference"))]
fn run_real_inference(
    _model_path: &PathBuf,
    _input: &[f32],
    _plugin_dir: Option<PathBuf>,
) -> Result<Vec<f32>> {
    anyhow::bail!(
        "binary was built without 'real-inference' feature. Rebuild with --features real-inference"
    )
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
        let bytes = fs::read(model)
            .with_context(|| format!("failed to read model bytes: {}", model.display()))?;
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
