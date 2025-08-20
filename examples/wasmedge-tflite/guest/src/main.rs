use anyhow::{anyhow, Context, Result};
use std::env;
use std::fs;

use bytemuck::{cast_slice, cast_slice_mut};
use wasi_nn::{ExecutionTarget, GraphBuilder, GraphEncoding, TensorType};

fn parse_args() -> Result<(String, Vec<f32>)> {
    // Very simple arg parsing: --model <path> --input "0.1,0.2,..."
    let mut args = env::args().skip(1);
    let mut model = None;
    let mut input = None;

    while let Some(flag) = args.next() {
        match flag.as_str() {
            "--model" => model = args.next(),
            "--input" => input = args.next(),
            _ => {}
        }
    }

    let model_path = model.ok_or_else(|| anyhow!("missing --model <path>"))?;
    let input_str = input.unwrap_or_else(|| "0.1,0.2,0.3,0.4".to_string());

    let mut values = Vec::new();
    for (i, tok) in input_str.split(',').enumerate() {
        let v: f32 = tok.trim().parse().with_context(|| format!("invalid float at pos {}: '{}'", i, tok))?;
        values.push(v);
    }
    if values.is_empty() {
        return Err(anyhow!("no inputs provided"));
    }
    Ok((model_path, values))
}

fn main() -> Result<()> {
    let (model_path, inputs) = parse_args()?;

    // Load model bytes
    let model = fs::read(&model_path)
        .with_context(|| format!("failed to read model: {}", model_path))?;

    // Build TFLite graph
    let builder = GraphBuilder::new(GraphEncoding::TensorflowLite, ExecutionTarget::CPU)
        .context("failed to create GraphBuilder")?;
    let graph = builder
        .build_from_bytes(&[&model])
        .context("failed to build graph from model bytes")?;

    // Prepare execution context
    let mut ctx = graph
        .init_execution_context()
        .context("failed to init execution context")?;

    // Set input tensor
    let dims = vec![1, inputs.len() as u32];
    let input_bytes: &[u8] = cast_slice(inputs.as_slice());
    ctx.set_input(0, TensorType::F32, input_bytes, &dims)
        .context("failed to set input tensor")?;

    // Compute
    ctx.compute().context("failed to compute graph")?;

    // Get output
    let mut out_buf = vec![0u8; 4096];
    let size = ctx
        .get_output(0, out_buf.as_mut_slice())
        .context("failed to get output")? as usize;

    out_buf.truncate(size);
    let out_f32: &[f32] = cast_slice_mut(out_buf.as_mut_slice());

    // Print as a simple CSV
    let s = out_f32
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",");
    println!("{}", s);

    Ok(())
}
