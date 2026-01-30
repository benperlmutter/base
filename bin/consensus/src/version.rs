//! Version information for the Base binary.

pub use base_cli_utils::VersionInfo;

/// Cargo package version.
pub const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Version info instance for metrics.
pub const VERSION: VersionInfo = VersionInfo::new(CARGO_PKG_VERSION);
