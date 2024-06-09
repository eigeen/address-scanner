pub mod memory_utils;
mod pattern_scan;

pub use memory_utils::MemoryUtils;

#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    #[error("pattern not found")]
    NotFound,
    #[error("more than one pattern found, expected exactly one")]
    MultipleMatchesFound,
    #[error("pattern scan error: {0}")]
    PatternScan(#[from] pattern_scan::Error),
}
