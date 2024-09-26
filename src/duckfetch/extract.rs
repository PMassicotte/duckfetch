use anyhow::{Context, Result};
use std::fs::File;
use std::path::{Path, PathBuf};
use tempfile::tempdir;
use zip::read::ZipArchive;

/// Extracts a ZIP file to the specified output directory.
///
/// # Arguments
///
/// * `file_path` - A `PathBuf` representing the path to the ZIP file.
/// * `output_dir` - A `Path` representing the path to the output directory.
///
/// # Returns
///
/// * `Result<()>` - An empty result if successful, or an error if extraction fails.
pub fn extract_zip(file_path: PathBuf, output_dir: &Path) -> Result<()> {
    let file = File::open(&file_path).context("Failed to open zip file")?;
    let mut archive = ZipArchive::new(file).context("Failed to read zip archive")?;

    // Not super happy with this. Find a better way to direcly find the duckdb binary file
    match archive.len() {
        // For the stable release
        1 => {
            archive
                .extract(output_dir)
                .context("Failed to extract zip archive")?;
            Ok(())
        }
        // For the nightly release because it contains nested zip files
        2 => {
            let mut nested_zip = archive
                .by_name("duckdb_cli-linux-amd64.zip")
                .context("Failed to find the specified file in the archive")?;

            let temp_dir = tempdir().context("Failed to create temporary directory")?;
            let temp_zip_path = temp_dir.path().join("duckdb.zip");

            let mut temp_zip_file =
                File::create(&temp_zip_path).context("Failed to create temporary zip file")?;
            std::io::copy(&mut nested_zip, &mut temp_zip_file)
                .context("Failed to copy contents to temporary zip file")?;

            let nested_file =
                File::open(&temp_zip_path).context("Failed to open nested zip file")?;
            let mut nested_archive =
                ZipArchive::new(nested_file).context("Failed to read nested zip archive")?;

            nested_archive
                .extract(output_dir)
                .context("Failed to extract nested zip archive")?;
            Ok(())
        }
        _ => Err(anyhow::anyhow!("Invalid zip file")),
    }
}
