use std::fs;
use std::io::{Error as IoError};

pub fn store(mails: Vec<String>) -> Result<Vec<String>, IoError> {
    let mut mbox_paths: Vec<String> = Vec::new();
    mails.iter().for_each(|mail| {
        let current_path = format!("tests/store/{}.mbox", mbox_paths.len());
        fs::write(current_path.clone(), mail).unwrap();
        mbox_paths.push(current_path);
    });
    Ok(mbox_paths)
}