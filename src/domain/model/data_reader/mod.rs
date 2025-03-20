use std::fmt::Debug;

pub trait DataReader: Debug {
    fn seek(&mut self, offset: u64);
    fn read(&mut self, length: u64, buffer: &mut String) -> usize;
}
