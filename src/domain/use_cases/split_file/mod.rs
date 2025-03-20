
use crate::ChunkError;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use crate::adapters::secondary::data_readers::file::FileReader;

const MBOX_MAIL_SEPARATOR: &str = "\nFrom ";

pub fn split_file(mut data_reader: FileReader, chunk_size: u64) -> Result<Vec<String>, ChunkError> {
    let mut mails: Vec<String> = Vec::new();
    let mut offset = 0;

    loop {
        let mut buffer = String::from("");
        data_reader.seek(offset);
        let bytes_read = data_reader.reader
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
