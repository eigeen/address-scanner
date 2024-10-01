pub mod memory_util;
mod pattern_scan;
mod windows_util;

pub use memory_util::MemoryUtils;

#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    #[error("pattern not found")]
    NotFound,
    #[error("more than one pattern found, expected exactly one")]
    MultipleMatchesFound,
    #[error("pattern scan error: {0}")]
    PatternScan(#[from] pattern_scan::Error),

    #[error("windows error: {0}")]
    Windows(#[from] windows::core::Error),
}
