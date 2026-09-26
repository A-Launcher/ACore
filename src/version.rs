/// Semantic API version used between A Launcher components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ApiVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl ApiVersion {
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self { major, minor, patch }
    }
}

/// Current public ACore contract version.
pub const CORE_API_VERSION: ApiVersion = ApiVersion::new(0, 1, 0);
