#[derive(Debug, PartialEq)]
enum Platform {
    Windows,
    MacOs,
    Linux,
}

#[derive(Debug, PartialEq)]
enum Architecture {
    Amd64,
    Arm64,
    Universal, // Added Universal architecture for MacOS
}

#[derive(Debug, PartialEq)]
enum BuildType<'a> {
    Stable(&'a str), // The tag name for stable builds
    Nightly,         // Nightly builds without a tag
}

trait AsStr {
    fn as_str(&self) -> &'static str;
}

impl AsStr for Platform {
    fn as_str(&self) -> &'static str {
        match self {
            Platform::Windows => "windows",
            Platform::MacOs => "osx",
            Platform::Linux => "linux",
        }
    }
}

impl AsStr for Architecture {
    fn as_str(&self) -> &'static str {
        match self {
            Architecture::Amd64 => "amd64",
            Architecture::Arm64 => "arm64",
            Architecture::Universal => "universal",
        }
    }
}

fn platform() -> Platform {
    if cfg!(target_os = "windows") {
        Platform::Windows
    } else if cfg!(target_os = "macos") {
        Platform::MacOs
    } else if cfg!(target_os = "linux") {
        Platform::Linux
    } else {
        panic!("Unsupported platform!")
    }
}

fn architecture(platform: &Platform) -> Architecture {
    match (
        cfg!(target_arch = "x86_64"),
        cfg!(target_arch = "aarch64"),
        platform,
    ) {
        (true, _, Platform::MacOs) => Architecture::Universal,
        (true, _, _) => Architecture::Amd64,
        (_, true, Platform::MacOs) => Architecture::Universal,
        (_, true, _) => Architecture::Arm64,
        _ => panic!("Unsupported architecture!"),
    }
}

pub fn build(tag_name: &str) -> String {
    const BASE_URL: &str = "https://github.com/duckdb/duckdb/releases/download/";
    const NIGHTLY_URL: &str = "https://artifacts.duckdb.org/latest/duckdb-binaries-";

    let build_type = if tag_name == "Nightly" {
        BuildType::Nightly
    } else {
        BuildType::Stable(tag_name)
    };

    let platform = platform();
    let architecture = architecture(&platform);

    match build_type {
        BuildType::Stable(tag_name) => format!(
            "{}{}/duckdb_cli-{}-{}.zip",
            BASE_URL,
            tag_name,
            platform.as_str(),
            architecture.as_str()
        ),
        BuildType::Nightly => {
            if platform == Platform::Linux && architecture == Architecture::Arm64 {
                format!("{}linux-aarch64.zip", NIGHTLY_URL)
            } else {
                format!("{}{}.zip", NIGHTLY_URL, platform.as_str())
            }
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
            "https://artifacts.duckdb.org/latest/duckdb-binaries-linux.zip",
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
