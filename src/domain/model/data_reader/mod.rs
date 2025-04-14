use std::fmt::{Debug, Display, Formatter};
use std::io::{Error, ErrorKind};
use async_trait::async_trait;

#[async_trait]
pub trait AsyncDataReader: Debug + Send + Sync {
    async fn seek(&mut self, offset: u64) -> Result<u64, Error>;
    async fn read(&mut self, length: u64, buffer: &mut String) -> Result<usize, Error>;
}


#[derive(Debug)]
pub enum ChunkError {
    SizeTooSmall,
}

impl Display for ChunkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkError::SizeTooSmall => write!(f, "Chunk size provided is too small, no separator found"),
        }
    }
}

impl std::error::Error for ChunkError {}

impl From<ChunkError> for Error {
    fn from(error: ChunkError) -> Self {
        Error::new(ErrorKind::InvalidInput, error.to_string())
    }
}