use anyhow::{Context, Result};
use reqwest::blocking::Client;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;
use tempfile::TempDir;

use crate::duckfetch::version::Release;

/// Downloads the DuckDB CLI for a specified version and saves it to a temporary directory.
///
/// # Arguments
///
/// * `version` - A string slice representing the version of DuckDB to download.
///
/// # Returns
///
/// * `Result<(PathBuf, TempDir)>` - A result containing a tuple with the path to the downloaded file and the temporary directory if successful, or an error.
pub fn download_duckdb(release: &Release) -> Result<(PathBuf, TempDir)> {
    let client = Client::new();

    let temp_dir = tempfile::tempdir().context("Failed to create temporary directory")?;

    // Get the file name from the url
    let temp_file_path = temp_dir
        .path()
        .join(release.url.clone().rsplit('/').next().unwrap());

    let mut temp_file = File::create(&temp_file_path).context("Failed to create temporary file")?;

    let mut response = client
        .get(&release.url)
        .send()
        .context("Failed to download file")?;

    copy(&mut response, &mut temp_file).context("Failed to copy content to temporary file")?;

    Ok((temp_file_path, temp_dir))
}

#[cfg(test)]
mod tests {
    use super::download_duckdb;
    use crate::duckdb_versions;

    #[test]
    fn test_download_duckdb() {
        let releases = duckdb_versions().unwrap();

        let tag_name = "v0.2.7";

        let release = releases.release_by_tag(tag_name).unwrap();

        let result = download_duckdb(release);

        assert!(result.is_ok());

        let (path, temp_dir) = result.unwrap();

        assert!(path.exists());

        drop(temp_dir);
    }
}
