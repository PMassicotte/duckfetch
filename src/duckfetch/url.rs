use crate::duckfetch::target::*;

/// Constructs the URL for downloading the specified build.
///
/// # Arguments
///
/// * `tag_name` - The tag name of the build. Use "Nightly" for nightly builds.
///
/// # Returns
///
/// * `String` - The URL for downloading the specified build.
pub fn build(tag_name: &str) -> String {
    // see urls: https://duckdb.org/docs/installation
    const BASE_URL: &str = "https://github.com/duckdb/duckdb/releases/download/";
    const NIGHTLY_URL: &str = "https://artifacts.duckdb.org/latest/duckdb-binaries-";

    let build_type = if tag_name == "Nightly" {
        BuildType::Nightly
    } else {
        BuildType::Stable(tag_name)
    };

    // TODO: Should be passed as reference
    let target = Target::new();

    match build_type {
        BuildType::Stable(tag_name) => format!(
            "{}{}/duckdb_cli-{}-{}.zip",
            BASE_URL,
            tag_name,
            target.platform.as_str(),
            target.architecture.as_str()
        ),
        BuildType::Nightly => {
            let platform_suffix = match target.platform {
                Platform::Linux => {
                    if target.architecture == Architecture::Arm64 {
                        "linux-arm64"
                    } else {
                        "linux-amd64"
                    }
                }
                Platform::MacOs => "osx",
                Platform::Windows => "windows",
            };
            format!("{}{}.zip", NIGHTLY_URL, platform_suffix)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::build;

    fn assert_build(tag_name: &str, expected: &str) {
        let result = build(tag_name);
        assert_eq!(result, expected);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_stable_linux_amd64() {
        assert_build(
            "v1.1.1",
            "https://github.com/duckdb/duckdb/releases/download/v1.1.1/duckdb_cli-linux-amd64.zip",
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_nightly_linux_amd64() {
        assert_build(
            "Nightly",
            "https://artifacts.duckdb.org/latest/duckdb-binaries-linux-amd64.zip",
        );
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_stable_windows_amd64() {
        assert_build(
            "v1.1.1",
            "https://github.com/duckdb/duckdb/releases/download/v1.1.1/duckdb_cli-windows-amd64.zip",
        );
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_nightly_windows() {
        assert_build(
            "Nightly",
            "https://artifacts.duckdb.org/latest/duckdb-binaries-windows.zip",
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_stable_macos() {
        assert_build(
            "v1.1.1",
            "https://github.com/duckdb/duckdb/releases/download/v1.1.1/duckdb_cli-osx-universal.zip",
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_nightly_macos() {
        assert_build(
            "Nightly",
            "https://artifacts.duckdb.org/latest/duckdb-binaries-osx.zip",
        );
    }

    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    #[test]
    fn test_nightly_linux_aarch64() {
        assert_build(
            "Nightly",
            "https://artifacts.duckdb.org/latest/duckdb-binaries-linux-aarch64.zip",
        );
    }
}
