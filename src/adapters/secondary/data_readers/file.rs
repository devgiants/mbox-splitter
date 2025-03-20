use std::fs::File;
use std::io::BufReader;

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
        
    }
    
}