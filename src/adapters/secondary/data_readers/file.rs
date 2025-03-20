use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};

pub struct FileReader {
    reader: BufReader<File>,
}

impl FileReader {
    pub fn new(file_path: &str) -> Self {
        Self {
            reader: BufReader::new(File::open(file_path).unwrap()),
        }
    }

    pub fn seek(&mut self, offset: u64) {
        self.reader.seek(SeekFrom::Start(offset)).unwrap();
    }

    pub fn read(&mut self, length: u64, mut buffer: &mut String) -> usize {
        let bytes_read = self
            .reader
            .by_ref()
            .take(length)
            .read_to_string(&mut buffer)
            .expect("Failed to read from file");
        bytes_read
    }
}
