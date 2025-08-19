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

        let mut request = client
            .get(url)
            .header(reqwest::header::USER_AGENT, "duckfetch");

        if let Ok(token) = std::env::var("GITHUB_TOKEN") {
            request = request.header(reqwest::header::AUTHORIZATION, format!("Bearer {}", token));
        }

        let response: ArtifactsResponse = request
            .send()
            .context("Failed to send request")?
            .json()?;

        Ok(response)
    }

    /// Find the creation date of the most recent nightly artifact with a name containing 'duckdb-binaries-linux'. This assumes that the first artifact is the most recently produced.
    ///
    /// # Returns
    ///
    /// * `Result<String>` - A result containing the creation date of the latest nightly artifact or an error.
    pub fn latest_nightly_date(&self) -> Result<String> {
        let created_at = self
            .artifacts
            .iter()
            .find(|artifact| artifact.name.contains("binaries")) // The
            // find() function return the first find
            .context("Could not find the creation date of the latest nightly build")?
            .created_at
            .clone();

        Ok(created_at)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_new() {
        // Mock the API response
        let mock_response = json!({
            "artifacts": [
                {
                    "name": "duckdb-binaries-linux",
                    "created_at": "2023-01-01T00:00:00Z"
                },
                {
                    "name": "other-artifact",
                    "created_at": "2023-01-02T00:00:00Z"
                }
            ]
        });

        // Deserialize the mock response
        let artifacts_response: ArtifactsResponse = serde_json::from_value(mock_response).unwrap();

        // Check that the deserialization was successful
        assert_eq!(artifacts_response.artifacts.len(), 2);
        assert_eq!(
            artifacts_response.artifacts[0].name,
            "duckdb-binaries-linux"
        );
        assert_eq!(
            artifacts_response.artifacts[0].created_at,
            "2023-01-01T00:00:00Z"
        );
    }

    #[test]
    fn test_latest_nightly_date() {
        // Mock the artifact list
        let artifacts = vec![
            Artifacts {
                name: "duckdb-binaries-linux".to_string(),
                created_at: "2023-01-01T00:00:00Z".to_string(),
            },
            Artifacts {
                name: "other-artifact".to_string(),
                created_at: "2023-01-02T00:00:00Z".to_string(),
            },
        ];

        let artifacts_response = ArtifactsResponse { artifacts };

        // Check that the latest nightly date is correct
        let result = artifacts_response.latest_nightly_date().unwrap();
        assert_eq!(result, "2023-01-01T00:00:00Z");
    }

    #[test]
    fn test_latest_nightly_date_no_match() {
        // Mock the artifact list with no matching artifacts
        let artifacts = vec![Artifacts {
            name: "other-artifact".to_string(),
            created_at: "2023-01-02T00:00:00Z".to_string(),
        }];

        let artifacts_response = ArtifactsResponse { artifacts };

        // Check that the function returns an error when no matching artifact is found
        let result = artifacts_response.latest_nightly_date();
        assert!(result.is_err());
    }
}
