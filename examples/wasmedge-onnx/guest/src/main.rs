use std::fs;
use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use clap::Parser;
use log::info;

#[derive(Parser, Debug)]
#[command(name = "onnx-guest")]
#[command(about = "WASI guest for ONNX inference via wasi-nn (feature-gated)", long_about = None)]
struct Args {
    /// Path to an .onnx model
    #[arg(short, long)]
    model: PathBuf,

    /// Comma-separated list of float inputs
    #[arg(short, long)]
    input: String,
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

#[cfg(feature = "onnx-real")]
fn run_onnx(model_path: &PathBuf, input: &[f32]) -> Result<Vec<f32>> {
    use wasi_nn::{
        ExecutionTarget, GraphBuilder, GraphEncoding, TensorType,
    };

    let model_bytes = fs::read(model_path)
        .with_context(|| format!("failed to read model bytes: {}", model_path.display()))?;

    // Build and load ONNX graph
    let graph = GraphBuilder::new(GraphEncoding::Onnx, ExecutionTarget::CPU)
        .build_from_bytes(&[&model_bytes])
        .context("failed to build ONNX graph")?;

    // Init execution context
    let mut ctx = graph.init_execution_context().context("failed to init exec ctx")?;

    // For simplicity, treat the input as a 1-D tensor of f32
    let dims = [input.len() as u32];
    let input_bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(
            input.as_ptr() as *const u8,
            input.len() * std::mem::size_of::<f32>(),
        )
    };
    ctx.set_input(0, TensorType::F32, &dims, input_bytes)
        .context("failed to set input tensor")?;

    // Compute
    ctx.compute().context("compute failed")?;

    // Get output 0 (assume 1-D f32 as well; adjust as needed for your model)
    // First query output size
    let mut out_buf = vec![0u8; 4 * input.len()];
    let nbytes = ctx.get_output(0, &mut out_buf).context("get_output failed")?;
    out_buf.truncate(nbytes);

    // Reinterpret as f32
    if out_buf.len() % 4 != 0 {
        bail!("output bytes not aligned to f32: {} bytes", out_buf.len());
    }
    let mut out = Vec::with_capacity(out_buf.len() / 4);
    for chunk in out_buf.chunks_exact(4) {
        let arr = [chunk[0], chunk[1], chunk[2], chunk[3]];
        out.push(f32::from_le_bytes(arr));
    }
    Ok(out)
}

#[cfg(not(feature = "onnx-real"))]
fn run_onnx(_model_path: &PathBuf, input: &[f32]) -> Result<Vec<f32>> {
    // Fallback: simple transform so this guest is runnable without wasi-nn.
    Ok(input.iter().map(|v| (v * 1.5).tanh()).collect())
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();
    let inputs = parse_inputs(&args.input)?;
    info!("guest inputs: {} values", inputs.len());

    let outputs = run_onnx(&args.model, &inputs)?;

    // Print CSV only to stdout; host will parse this.
    let csv = outputs
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",");
    println!("{}", csv);
    Ok(())
}
