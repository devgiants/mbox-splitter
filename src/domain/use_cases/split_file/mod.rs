use crate::domain::model::data_reader::DataReader;
use crate::domain::model::data_reader::ChunkError;
use std::io::Error;

const MBOX_MAIL_SEPARATOR: &str = "\nFrom ";

pub fn split_file(mut data_reader: Box<dyn DataReader>, chunk_size: u64) -> Result<Vec<String>, Error> {
    let mut mails: Vec<String> = Vec::new();
    let mut offset = 0;

    loop {
        let mut buffer = String::from("");
        data_reader.seek(offset)?;
        let bytes_read = data_reader.read(chunk_size, &mut buffer)?;

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
            return Err(Error::from(ChunkError::SizeTooSmall));
        }
    }

    Ok(mails)
}
