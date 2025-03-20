use std::fmt::Debug;
use std::io::Error;

pub trait DataReader: Debug {
    fn seek(&mut self, offset: u64) -> Result<u64, Error>;
    fn read(&mut self, length: u64, buffer: &mut String) -> Result<usize, Error>;
}
