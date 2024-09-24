use anyhow::{Context, Result};
use duckup::duckup::{download_duckdb, extract_zip, get_latest_release, install_duckdb};

// use reqwest::blocking::Client;
/// Main function to orchestrate the download, extraction, and installation of DuckDB.
///
/// # Returns
///
/// * `Result<()>` - An empty result, or an error if any step fails.
fn main() -> Result<()> {
    let version = get_latest_release()?;

    println!("Latest DuckDB version: {}", version);

    let (downloaded_file, temp_dir) = download_duckdb(&version)?;

    let temp_dir_str = temp_dir
        .path()
        .to_str()
        .context("Failed to convert temp_dir path to str")?;

    extract_zip(downloaded_file, temp_dir_str)?;
    install_duckdb(temp_dir_str, "/usr/local/bin")?;

    println!("DuckDB installed successfully!");

    Ok(())
}
