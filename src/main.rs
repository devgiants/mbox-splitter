mod domain;
mod adapters;

use std::io::Error;

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::use_cases::split_file::split_file;
    use crate::domain::use_cases::store_mails::store;
    use crate::adapters::secondary::data_readers::file::FileReader;
    use std::{fs, io};

    const TEST_STORAGE_DIR: &str = "tests/store";
    #[test]
    fn should_produce_no_mail() -> io::Result<()> {
        
        let file_path = "tests/data/empty.mbox";
        let data_reader = FileReader::new(file_path);
        let mails: Vec<String> = split_file(data_reader, 80000)?;
        assert_eq!(mails.len(), 0);
        Ok(())
    }

    #[test]
    fn should_produce_one_mail() -> io::Result<()> {
        let file_path = "tests/data/one_mail.mbox";
        let data_reader = FileReader::new(file_path);
        let mails: Vec<String> = split_file(data_reader, 100000)?;
        assert_eq!(mails.len(), 1);
        Ok(())
    }

    #[test]
    fn should_rise_error_regarding_chunk_size() -> io::Result<()> {
        let file_path = "tests/data/one_mail.mbox";
        let data_reader = FileReader::new(file_path);
        let mails = split_file(data_reader, 100);
        assert_eq!(mails.is_err(), true);
        Ok(())
    }

    #[test]
    fn should_produce_two_mails() -> io::Result<()> {
        let file_path = "tests/data/two_mails.mbox";
        let data_reader = FileReader::new(file_path);
        let mails: Vec<String> = split_file(data_reader, 1200)?;
        assert_eq!(mails.len(), 2);
        Ok(())
    }

    #[test]
    fn should_produce_four_mails() -> io::Result<()> {
        let file_path = "tests/data/100_mails.mbox";
        let data_reader = FileReader::new(file_path);
        let mails: Vec<String> = split_file(data_reader, 30000)?;
        assert_eq!(mails.len(), 4);
        Ok(())
    }

    #[test]
    fn should_store_four_mails() -> io::Result<()> {
        reinit_storage_dir()?;
        let file_path = "tests/data/100_mails.mbox";
        let data_reader = FileReader::new(file_path);
        let mails: Vec<String> = split_file(data_reader, 30000)?;
        store(mails, "tests/store")?;
        for i in 0..=3 {
            assert_eq!(
                fs::exists(format!("{}/{}.mbox", TEST_STORAGE_DIR, i))?,
                true
            );
        }
        Ok(())
    }

    #[test]
    fn should_rise_error_on_storage_destination() -> io::Result<()> {
        reinit_storage_dir()?;
        let file_path = "tests/data/100_mails.mbox";
        let data_reader = FileReader::new(file_path);
        let mails: Vec<String> = split_file(data_reader, 30000)?;
        let storage_return = store(mails, "/");
        assert_eq!(storage_return.is_err(), true);
        Ok(())
    }

    fn reinit_storage_dir() -> Result<(), Error> {
        fs::remove_dir_all("tests/store")?;
        fs::create_dir("tests/store")?;
        Ok(())
    }
}
