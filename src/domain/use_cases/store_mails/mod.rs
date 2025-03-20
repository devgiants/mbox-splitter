use std::fs;
use std::io::{Error as IoError};

pub fn store(mails: Vec<String>, root_path: &str) -> Result<Vec<String>, IoError> {
    mails.into_iter().enumerate().map(|(i, mail)| {
        let current_path = format!("{}/{}.mbox", root_path, i);
        fs::write(&current_path, &mail).map(|_| current_path)
    }).collect()
}