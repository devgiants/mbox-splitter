use std::fs;
use std::fs::File;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::{BufReader, Error as IoError, ErrorKind, Read, Seek, SeekFrom};

#[derive(Debug)]
enum ChunkError {
    SizeTooSmall,
}

impl Display for ChunkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkError::SizeTooSmall => write!(f, "Chunk size provided is too small, no separator found"),
        }
    }
}

impl Error for ChunkError {}

impl From<ChunkError> for IoError {
    fn from(error: ChunkError) -> Self {
        IoError::new(ErrorKind::InvalidInput, error.to_string())
    }
}

fn main() {}

const MBOX_MAIL_SEPARATOR: &str = "\nFrom ";

fn split_file(file_reader_ref: &mut File, chunk_size: u64) -> Result<Vec<String>, ChunkError> {
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
            // mails.push(buffer);
            // offset += bytes_read as u64;
        }
    }

    Ok(mails)
}

fn store(mails: Vec<String>) -> Result<Vec<String>, IoError> {
    let mut mbox_paths: Vec<String> = Vec::new();
    mails.iter().for_each(|mail| {
        let current_path = format!("tests/store/{}.mbox", mbox_paths.len());
        fs::write(current_path.clone(), mail).unwrap();
        mbox_paths.push(current_path);
    });
    Ok(mbox_paths)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::{fs, io};

    const TEST_STORAGE_DIR: &str = "tests/store";
    #[test]
    fn should_produce_no_mail() -> io::Result<()> {
        let file_path = "tests/data/empty.mbox";
        let mut file_reader_ref = File::open(file_path)?;
        let mails: Vec<String> = split_file(&mut file_reader_ref, 80000)?;
        assert_eq!(mails.len(), 0);
        Ok(())
    }

    #[test]
    fn should_produce_one_mail() -> io::Result<()> {
        let file_path = "tests/data/one_mail.mbox";
        let mut file_reader_ref = File::open(file_path)?;
        let mails: Vec<String> = split_file(&mut file_reader_ref, 100000)?;
        assert_eq!(mails.len(), 1);
        Ok(())
    }

    #[test]
    fn should_rise_error_regarding_chunk_size() -> io::Result<()> {
        let file_path = "tests/data/one_mail.mbox";
        let mut file_reader_ref = File::open(file_path)?;
        let mails = split_file(&mut file_reader_ref, 100);
        assert_eq!(mails.is_err(), true);
        Ok(())
    }

    #[test]
    fn should_produce_two_mails() -> io::Result<()> {
        let file_path = "tests/data/two_mails.mbox";
        let mut file_reader_ref = File::open(file_path)?;
        let mails: Vec<String> = split_file(&mut file_reader_ref, 1200)?;
        assert_eq!(mails.len(), 2);
        Ok(())
    }

    #[test]
    fn should_produce_four_mails() -> io::Result<()> {
        let file_path = "tests/data/100_mails.mbox";
        let mut file_reader_ref = File::open(file_path)?;
        let mails: Vec<String> = split_file(&mut file_reader_ref, 30000)?;
        assert_eq!(mails.len(), 4);
        Ok(())
    }

    #[test]
    fn should_store_four_mails() -> io::Result<()> {
        reinit_storage_dir()?;
        let file_path = "tests/data/100_mails.mbox";
        let mut file_reader_ref = File::open(file_path)?;
        let mails: Vec<String> = split_file(&mut file_reader_ref, 30000)?;
        store(mails)?;
        for i in 0..=3 {
            assert_eq!(
                fs::exists(format!("{}/{}.mbox", TEST_STORAGE_DIR, i))?,
                true
            );
        }
        Ok(())
    }

    fn reinit_storage_dir() -> Result<(), IoError> {
        fs::remove_dir_all("tests/store")?;
        fs::create_dir("tests/store")?;
        Ok(())
    }
}
