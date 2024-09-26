use anyhow::{Context, Result};
use reqwest::blocking::Client;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;
use tempfile::TempDir;

/// Downloads the DuckDB CLI for a specified version and saves it to a temporary directory.
///
/// # Arguments
///
/// * `version` - A string slice representing the version of DuckDB to download.
///
/// # Returns
///
/// * `Result<(PathBuf, TempDir)>` - A result containing a tuple with the path to the downloaded file and the temporary directory if successful, or an error.
pub fn download_duckdb(version: &str) -> Result<(PathBuf, TempDir)> {
    let client = Client::new();

    let url = format!(
        "https://github.com/duckdb/duckdb/releases/download/{}/duckdb_cli-linux-amd64.zip",
        version
    );

    let temp_dir = tempfile::tempdir().context("Failed to create temporary directory")?;
    let temp_file_path = temp_dir.path().join("duckdb_cli-linux-amd64.zip");
    let mut temp_file = File::create(&temp_file_path).context("Failed to create temporary file")?;

    let mut response = client.get(&url).send().context("Failed to download file")?;
    copy(&mut response, &mut temp_file).context("Failed to copy content to temporary file")?;

    Ok((temp_file_path, temp_dir))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_duckdb() {
        let version = "v0.2.7";
        let result = download_duckdb(version);

        assert!(result.is_ok());

        let (path, temp_dir) = result.unwrap();
        assert!(path.exists());

        // Clean up the temporary directory
        drop(temp_dir);
    }
}
