use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn install_duckdb(output_dir: &str, install_dir: &str) -> Result<()> {
    let src = Path::new(output_dir).join("duckdb");
    let dest_path = Path::new(install_dir).join("duckdb");
    fs::rename(src, dest_path).context("Failed to move DuckDB binary")?;
    Ok(())
}
