use anyhow::{Context, Result};
use std::fs::File;
use std::path::{Path, PathBuf};
use zip::read::ZipArchive;

/// Extracts a ZIP file to the specified output directory.
///
/// # Arguments
///
/// * `file_path` - A `PathBuf` representing the path to the ZIP file.
/// * `output_dir` - A string slice representing the path to the output directory.
///
/// # Returns
///
/// * `Result<()>` - An empty result if successful, or an error if extraction fails.
pub fn extract_zip(file_path: PathBuf, output_dir: &Path) -> Result<()> {
    let file = File::open(file_path).context("Failed to open zip file")?;
    let mut archive = ZipArchive::new(file).context("Failed to read zip archive")?;
    archive
        .extract(output_dir)
        .context("Failed to extract zip archive")?;
    Ok(())
}
