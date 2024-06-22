pub use address::MemoryUtils;

pub use macros::hex_str_to_bytes;
pub use macros::AddressRecord;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("memory module error: {0}")]
    Memory(#[from] address::MemoryError),
    #[error("more than one pattern found, expected exactly one")]
    MultipleMatchesFound,
}

pub trait AddressProvider: 'static + Send + Sync {
    fn get_address(&self) -> Result<usize, Error>;

    fn name(&self) -> &'static str;
}
