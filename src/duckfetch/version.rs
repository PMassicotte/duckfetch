use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::process::Command;
use std::str;

/// Represents a single release with a tag name and publication date.
#[derive(Serialize, Deserialize, Debug)]
pub struct Release {
    pub tag_name: String,
    pub published_at: String,
    pub url: String,
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

    pub fn releases(&self) -> Vec<String> {
        let releases: Vec<String> = self.releases.iter().map(|r| r.tag_name.clone()).collect();

        releases
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

    /// Gets a release by its tag name.
    pub fn get_release_by_tag(&self, tag_name: &str) -> Option<&Release> {
        self.releases
            .iter()
            .find(|release| release.tag_name == tag_name)
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
        .header("User-Agent", "duckfetch")
        .send()
        .context("Failed to send request")?
        .json()?;

    // Create a ReleaseCollection and populate it with the releases
    let mut release_collection = ReleaseCollection::new();

    // Add the nightly version
    release_collection.add_release(Release {
        tag_name: "Nightly".to_string(),
        published_at: "yyyy-mm-dd".to_string(),
        url: "https://artifacts.duckdb.org/latest/duckdb-binaries-linux.zip".to_string(),
    });

    for mut release in response {
        release.url = format!(
            "https://github.com/duckdb/duckdb/releases/download/{}/duckdb_cli-linux-amd64.zip",
            release.tag_name
        );
        release_collection.add_release(release);
    }

    Ok(release_collection)
}

/// Fetches the latest DuckDB release version from the GitHub API.
///
/// # Returns
///
/// * `Result<String>` - A result containing the latest version as a string if successful, or an error.
pub fn latest_stable_release() -> Result<String> {
    let url = "https://api.github.com/repos/duckdb/duckdb/releases/latest";

    let client = Client::new();

    let response = client
        .get(url)
        .header("User-Agent", "duckfetch")
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

/// Checks the installed version of DuckDB and compares it with the latest release.
///
/// This function runs the `duckdb --version` command to get the installed version of DuckDB.
/// It then compares the installed version with the latest release version obtained from
/// `get_latest_release()`. If the installed version is the latest, it prints a confirmation message.
/// Otherwise, it informs the user that a newer version is available.
///
/// # Errors
///
/// This function will print error messages if:
/// - The `duckdb --version` command fails to run.
/// - The output of the command cannot be converted to a string.
/// - The installed version cannot be extracted from the command output.
///
/// # Returns
///
/// This function returns `Ok(())` if the check completes without encountering any errors.
pub fn check() -> Result<()> {
    if let Ok(output) = Command::new("duckdb").arg("--version").output() {
        if output.status.success() {
            if let Ok(installed_version) = str::from_utf8(&output.stdout)
                .map(|s| s.split_whitespace().next().unwrap().to_string())
            {
                let latest_release = latest_stable_release()?;

                if installed_version == latest_release {
                    println!(
                        "The latest stable release of DuckDB is installed ({})",
                        latest_release
                    );
                } else if installed_version.contains("dev") {
                    println!(
                        "Nightly version installed: {}\nLatest stable version: {}",
                        installed_version, latest_release
                    );
                } else {
                    println!(
                        "A newer version of DuckDB is available.\nInstalled version: {}\nLatest stable version: {}",
                        installed_version,
                        latest_release,
                    );
                }
            } else {
                eprintln!("Failed to convert output to string");
            }
        } else {
            eprintln!("Failed to extract DuckDB version");
        }
    } else {
        eprintln!("Failed to run `duckdb --version`. Is it currently installed?");
    }

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_add_release() {
//         let mut collection = ReleaseCollection::new();
//         let release = Release {
//             tag_name: "v1.0.0".to_string(),
//             published_at: "2023-01-01T00:00:00Z".to_string(),
//         };
//         collection.add_release(release);
//
//         assert_eq!(collection.releases.len(), 1);
//         assert_eq!(collection.releases[0].tag_name, "v1.0.0");
//     }
//
//     #[test]
//     fn test_contains_version() {
//         let mut collection = ReleaseCollection::new();
//         let release = Release {
//             tag_name: "v1.0.0".to_string(),
//             published_at: "2023-01-01T00:00:00Z".to_string(),
//         };
//         collection.add_release(release);
//
//         assert!(collection.contains_version("v1.0.0"));
//         assert!(!collection.contains_version("v2.0.0"));
//     }
// }
