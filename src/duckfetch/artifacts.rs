use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Artifacts {
    name: String,
    created_at: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ArtifactsResponse {
    artifacts: Vec<Artifacts>,
}

impl ArtifactsResponse {
    pub fn new() -> Result<Self> {
        let client = Client::new();
        let url = "https://api.github.com/repos/duckdb/duckdb/actions/artifacts";

        let response: ArtifactsResponse = client
            .get(url)
            .header(reqwest::header::USER_AGENT, "duckfetch")
            .send()
            .context("Failed to send request")?
            .json()?;

        Ok(response)
    }

    pub fn latest_nightly_date(&self) -> Result<String> {
        let created_at = self
            .artifacts
            .iter()
            .find(|artifact| artifact.name.contains("duckdb-binaries-linux"))
            .context("No artifact found with name containing 'duckdb-binaries-linux'")?
            .created_at
            .clone();

        Ok(created_at)
    }
}
