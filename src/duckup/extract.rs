use anyhow::{Context, Result};
use std::fs::File;
use std::path::PathBuf;
use zip::read::ZipArchive;

pub fn extract_zip(file_path: PathBuf, output_dir: &str) -> Result<()> {
    let file = File::open(file_path).context("Failed to open zip file")?;
    let mut archive = ZipArchive::new(file).context("Failed to read zip archive")?;
    archive
        .extract(output_dir)
        .context("Failed to extract zip archive")?;
    Ok(())
}
