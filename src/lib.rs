//! ACore: shared contracts and primitives for the A Launcher ecosystem.
//!
//! ACore intentionally stays small and dependency-light. Higher-level
//! repositories should depend on these stable contracts rather than sharing
//! implementation details.

pub mod error;
pub mod launch;
pub mod platform;
pub mod version;

pub use error::{CoreError, CoreResult};
pub use launch::{LaunchRequest, LaunchResult};
pub use platform::{Architecture, Platform, PlatformInfo};
pub use version::{ApiVersion, CORE_API_VERSION};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_api_version_is_defined() {
        assert_eq!(CORE_API_VERSION.major, 0);
        assert_eq!(CORE_API_VERSION.minor, 1);
    }

    #[test]
    fn launch_request_can_be_constructed() {
        let request = LaunchRequest::new(
            "/game",
            "/runtime/bin/java",
            "net.minecraft.client.main.Main",
        );

        assert_eq!(request.game_dir, "/game");
        assert_eq!(request.java_executable, "/runtime/bin/java");
        assert_eq!(request.main_class, "net.minecraft.client.main.Main");
    }
}
