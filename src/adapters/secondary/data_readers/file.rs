use std::fs::File;
use std::io::{BufReader, SeekFrom, Seek};

pub struct FileReader {
    pub reader: BufReader<File>,
}


impl FileReader {
    pub fn new(file_path: &str) -> Self {
        Self {
            reader: BufReader::new(File::open(file_path).unwrap()),
        }
    }
    
    pub fn seek(&mut self, offset: u64)
    {
        self.reader.seek(SeekFrom::Start(offset)).unwrap();
    }
    
}