use anyhow::{Context, Result};
use reqwest::blocking::Client;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;
use tempfile::TempDir;

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

    println!("Downloaded file to: {:?}", temp_file_path);

    Ok((temp_file_path, temp_dir))
}
