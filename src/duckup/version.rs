use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Release {
    tag_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReleaseCollection {
    releases: Vec<Release>,
}

impl ReleaseCollection {
    // Method to create a new ReleaseCollection
    fn new() -> Self {
        ReleaseCollection {
            releases: Vec::new(),
        }
    }

    // Method to add a release to the collection
    fn add_release(&mut self, release: Release) {
        self.releases.push(release);
    }

    pub fn print_versions(&self) {
        for release in &self.releases {
            println!("{}", release.tag_name);
        }
    }

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

pub fn duckdb_versions() -> Result<ReleaseCollection> {
    let url = "https://api.github.com/repos/duckdb/duckdb/releases";
    let client = Client::new();

    // Set the User-Agent header
    let response: Vec<Release> = client
        .get(url)
        .header("User-Agent", "duckup")
        .send()?
        .json()?;

    // Create a ReleaseCollection and populate it with the releases
    let mut release_collection = ReleaseCollection::new();
    for release in response {
        release_collection.add_release(release);
    }

    Ok(release_collection)
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
