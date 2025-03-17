
use crate::ChunkError;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
const MBOX_MAIL_SEPARATOR: &str = "\nFrom ";

pub fn split_file(file_reader_ref: &mut File, chunk_size: u64) -> Result<Vec<String>, ChunkError> {
    let mut mails: Vec<String> = Vec::new();
    let mut reader = BufReader::new(file_reader_ref);
    let mut offset = 0;

    loop {
        let mut buffer = String::from("");
        reader.seek(SeekFrom::Start(offset)).unwrap();
        let bytes_read = reader
            .by_ref()
            .take(chunk_size)
            .read_to_string(&mut buffer)
            .expect("Failed to read from file");

        if bytes_read == 0 {
            break;
        }
        if bytes_read < chunk_size as usize {
            mails.push(buffer);
            break;
        }
        if let Some(last_separator_position) = buffer.rfind(MBOX_MAIL_SEPARATOR) {
            mails.push(buffer[0..last_separator_position].to_string());
            offset += last_separator_position as u64;
        } else {
            return Err(ChunkError::SizeTooSmall);
        }
    }

    Ok(mails)
}
