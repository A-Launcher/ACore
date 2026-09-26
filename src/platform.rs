/// Operating-system family visible to the launcher runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Android,
    Linux,
    Windows,
    MacOs,
    Unknown,
}

/// CPU architecture used by native runtime components.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Architecture {
    Arm64,
    Arm32,
    X86_64,
    X86,
    Unknown,
}

/// Runtime platform information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformInfo {
    pub platform: Platform,
    pub architecture: Architecture,
}

impl PlatformInfo {
    pub const fn new(platform: Platform, architecture: Architecture) -> Self {
        Self {
            platform,
            architecture,
        }
    }
}
