use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a single release with a tag name and publication date.
#[derive(Serialize, Deserialize, Debug)]
pub struct Release {
    tag_name: String,
    published_at: String,
}

/// A collection of releases.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReleaseCollection {
    releases: Vec<Release>,
}

impl ReleaseCollection {
    /// Creates a new, empty `ReleaseCollection`.
    fn new() -> Self {
        ReleaseCollection {
            releases: Vec::new(),
        }
    }

    /// Adds a release to the collection.
    ///
    /// # Arguments
    ///
    /// * `release` - A `Release` to add to the collection.
    fn add_release(&mut self, release: Release) {
        self.releases.push(release);
    }

    /// Prints the versions of all releases in the collection.
    pub fn print_versions(&self) {
        for release in &self.releases {
            println!(
                "{:<8} ({})",
                release.tag_name,
                release.published_at.split('T').next().unwrap()
            );
        }
    }

    /// Checks if the collection contains a release with the specified version.
    ///
    /// # Arguments
    ///
    /// * `version` - A string slice that holds the version to search for.
    ///
    /// # Returns
    ///
    /// * `true` if the collection contains the version, `false` otherwise.
    pub fn contains_version(&self, version: &str) -> bool {
        self.releases
            .iter()
            .any(|release| release.tag_name == version)
    }
}

// Implement IntoIterator for ReleaseCollection
impl IntoIterator for ReleaseCollection {
    type Item = Release;
    type IntoIter = std::vec::IntoIter<Release>;

    fn into_iter(self) -> Self::IntoIter {
        self.releases.into_iter()
    }
}

// Implement IntoIterator for a reference to ReleaseCollection (for borrowing)
impl<'a> IntoIterator for &'a ReleaseCollection {
    type Item = &'a Release;
    type IntoIter = std::slice::Iter<'a, Release>;

    fn into_iter(self) -> Self::IntoIter {
        self.releases.iter()
    }
}

/// Fetches the list of DuckDB releases from the GitHub API.
///
/// # Returns
///
/// * `Result<ReleaseCollection>` - A result containing the `ReleaseCollection` if successful, or an error.
pub fn duckdb_versions() -> Result<ReleaseCollection> {
    let url = "https://api.github.com/repos/duckdb/duckdb/releases";
    let client = Client::new();

    // Set the User-Agent header
    let response: Vec<Release> = client
        .get(url)
        .header("User-Agent", "duckup")
        .send()
        .context("Failed to send request")?
        .json()?;

    // Create a ReleaseCollection and populate it with the releases
    let mut release_collection = ReleaseCollection::new();
    for release in response {
        release_collection.add_release(release);
    }

    Ok(release_collection)
}

/// Fetches the latest DuckDB release version from the GitHub API.
///
/// # Returns
///
/// * `Result<String>` - A result containing the latest version as a string if successful, or an error.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_release() {
        let mut collection = ReleaseCollection::new();
        let release = Release {
            tag_name: "v1.0.0".to_string(),
            published_at: "2023-01-01T00:00:00Z".to_string(),
        };
        collection.add_release(release);

        assert_eq!(collection.releases.len(), 1);
        assert_eq!(collection.releases[0].tag_name, "v1.0.0");
    }

    #[test]
    fn test_contains_version() {
        let mut collection = ReleaseCollection::new();
        let release = Release {
            tag_name: "v1.0.0".to_string(),
            published_at: "2023-01-01T00:00:00Z".to_string(),
        };
        collection.add_release(release);

        assert!(collection.contains_version("v1.0.0"));
        assert!(!collection.contains_version("v2.0.0"));
    }
}
