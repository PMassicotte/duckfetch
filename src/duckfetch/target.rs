/// Represents a target platform and architecture.
#[derive(Debug, Clone, Copy)]
pub struct Target {
    pub platform: Platform,
    pub architecture: Architecture,
}

impl Target {
    /// Creates a new `Target` by detecting the current platform and architecture.
    pub fn new() -> Self {
        let platform = Platform::detect_platform();
        let architecture = Architecture::detect_architecture(&platform);
        Target {
            platform,
            architecture,
        }
    }
}

/// Represents the supported platforms.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Platform {
    Windows,
    MacOs,
    Linux,
}

impl Platform {
    /// Detects the current platform.
    ///
    /// # Panics
    ///
    /// Panics if the platform is not supported.
    pub fn detect_platform() -> Platform {
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
}

/// Represents the supported architectures.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Architecture {
    Amd64,
    Arm64,
    Universal, // Added Universal architecture for MacOS
}

impl Architecture {
    /// Detects the architecture based on the given platform.
    ///
    /// # Panics
    ///
    /// Panics if the architecture is not supported.
    pub fn detect_architecture(platform: &Platform) -> Architecture {
        match cfg!(target_arch = "x86_64") {
            true => match platform {
                Platform::MacOs => Architecture::Universal,
                _ => Architecture::Amd64,
            },
            false => match cfg!(target_arch = "aarch64") {
                true => match platform {
                    Platform::MacOs => Architecture::Universal,
                    _ => Architecture::Arm64,
                },
                false => panic!("Unsupported architecture!"),
            },
        }
    }
}

/// Represents the build type.
#[derive(Debug, PartialEq)]
pub enum BuildType<'a> {
    Stable(&'a str),
    Nightly,
}

/// A trait to convert an enum to a string slice.
pub trait AsStr {
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
