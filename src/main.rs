fn main() {}

const MBOX_MAIL_SEPARATOR: &str = "\nFrom ";

fn split(mails_content: &str, chunk_size: usize) -> Vec<String> {
    if mails_content.len() == 0 {
        return vec![];
    }
    let mut mails: Vec<String> = Vec::new();
    let mut current_length: usize = 0;
    let mut mails_chunked = String::new();

    mails_content.split(MBOX_MAIL_SEPARATOR).for_each(|mail| {
         if current_length > 0 && current_length + mail.len() > chunk_size {
             mails.push(mails_chunked.clone());
             mails_chunked = String::new();
             current_length = 0;
         }
         mails_chunked += mail;
         current_length += mail.len()
    });
     if current_length > 0 {
         mails.push(MBOX_MAIL_SEPARATOR.to_owned() + &*mails_chunked.clone());
     }
    mails
}

#[cfg(test)]
mod tests {
    use std::{fs, io};
    use super::*;

    #[test]
    fn empty_content_test() {
        let mails: Vec<String> = split("", 0);
        assert_eq!(mails.len(), 0);
    }

    #[test]
    fn one_mail_test() -> io::Result<()> {
        let mail_content = fs::read_to_string("tests/data/one_mail.mbox")?;
        let mails: Vec<String> = split(&mail_content, 100000);
        assert_eq!(mails.len(), 1);
        Ok(())
    }

    #[test]
    fn two_mails_test() -> io::Result<()> {
        let mail_content = fs::read_to_string("tests/data/two_mails.mbox")?;
        let mails: Vec<String> = split(&mail_content, 800);
        assert_eq!(mails.len(), 2);
        Ok(())
    }

    #[test]
    fn split_by_size_test() -> io::Result<()> {
        let mail_content = fs::read_to_string("tests/data/100_mails.mbox")?;
        let mails: Vec<String> = split(&mail_content, 80000);
        assert_eq!(mails.len(), 2);
        Ok(())
    }
}

