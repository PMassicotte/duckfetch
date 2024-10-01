use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
/// Represents an artifact with a name and creation date.
pub struct Artifacts {
    name: String,
    created_at: String,
}

#[derive(Debug, Deserialize, Clone)]
/// Represents a response containing a list of artifacts.
pub struct ArtifactsResponse {
    artifacts: Vec<Artifacts>,
}

impl ArtifactsResponse {
    /// Fetches the list of artifacts from the GitHub API.
    ///
    /// # Returns
    ///
    /// * `Result<ArtifactsResponse>` - A result containing the ArtifactsResponse or an error.
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

    /// Finds the creation date of the latest nightly artifact with the name containing 'duckdb-binaries-linux'. This is assuming that the first "artifactf" is the latest produced.
    ///
    /// # Returns
    ///
    /// * `Result<String>` - A result containing the creation date of the latest nightly artifact or an error.
    pub fn latest_nightly_date(&self) -> Result<String> {
        let created_at = self
            .artifacts
            .iter()
            .find(|artifact| artifact.name.contains("duckdb-binaries-linux")) // The
            // find() function return the first find
            .context("No artifact found with name containing 'duckdb-binaries-linux'")?
            .created_at
            .clone();

        Ok(created_at)
    }
}
