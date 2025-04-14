use crate::domain::model::data_reader::AsyncDataReader;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufReader, Error, Read, Seek, SeekFrom};
use async_trait::async_trait;

pub struct FileReader {
    reader: BufReader<File>,
}

impl FileReader {
    pub fn new(file_path: &str) -> Box<Self> {
        Box::new(Self {
            reader: BufReader::new(File::open(file_path).unwrap()),
        })
    }
}

impl Debug for FileReader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("FileReader")
    }
}

#[async_trait]
impl AsyncDataReader for FileReader {
    async fn seek(&mut self, offset: u64) -> Result<u64, Error> {
        self.reader.seek(SeekFrom::Start(offset))
    }

    async fn read(&mut self, length: u64, mut buffer: &mut String) -> Result<usize, Error> {
        let bytes_read = self
            .reader
            .by_ref()
            .take(length)
            .read_to_string(&mut buffer);
        bytes_read
    }
}
