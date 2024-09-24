use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Installs the DuckDB binary by moving it from the output directory to the install directory.
///
/// # Arguments
///
/// * `output_dir` - A `Path` representing the path to the directory containing the DuckDB binary.
/// * `install_dir` - A `Path` representing the path to the directory where the DuckDB binary should be installed.
///
/// # Returns
///
/// * `Result<()>` - An empty result if successful, or an error if the installation fails.
pub fn install_duckdb(temp_unzip_dir: &Path, dest_path: &Path) -> Result<()> {
    let src = Path::new(temp_unzip_dir).join("duckdb");
    let dest_path = Path::new(dest_path).join("duckdb");

    fs::rename(src, dest_path).context("Failed to move DuckDB binary")?;

    Ok(())
}
