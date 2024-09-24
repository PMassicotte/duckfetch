use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Installs the DuckDB binary by moving it from the output directory to the install directory.
///
/// # Arguments
///
/// * `output_dir` - A string slice representing the path to the directory containing the DuckDB binary.
/// * `install_dir` - A string slice representing the path to the directory where the DuckDB binary should be installed.
///
/// # Returns
///
/// * `Result<()>` - An empty result if successful, or an error if the installation fails.
pub fn install_duckdb(output_dir: &str, install_dir: &str) -> Result<()> {
    let src = Path::new(output_dir).join("duckdb");
    let dest_path = Path::new(install_dir).join("duckdb");
    fs::rename(src, dest_path).context("Failed to move DuckDB binary")?;
    Ok(())
}
