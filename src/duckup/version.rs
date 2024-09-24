use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct Release {
    tag_name: String,
}

pub fn list_all_versions() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.github.com/repos/duckdb/duckdb/releases";
    let client = Client::new();

    // Set the User-Agent header
    let response = client
        .get(url)
        .header("User-Agent", "duckup")
        .send()?
        .json::<Vec<Release>>()?;

    println!("DuckDB Versions:");
    for release in response {
        println!("{}", release.tag_name);
    }

    Ok(())
}

pub fn get_latest_release() -> Result<String> {
    let url = "https://api.github.com/repos/duckdb/duckdb/releases/latest";

    let client = Client::new();

    let response = client
        .get(url)
        .header("User-Agent", "duckup")
        .send()
        .context("Failed to send request")?
        .text()
        .context("Failed to read response text")?;

    let json: Value = serde_json::from_str(&response).context("Failed to parse JSON")?;
    let version = json["tag_name"]
        .as_str()
        .context("Could not find the 'tag_name' field in the response")?;

    Ok(version.to_string())
}
