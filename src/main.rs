fn main() {}

const MBOX_MAIL_SEPARATOR: &str = "\nFrom ";

fn split(mails_content: &str) -> Vec<&str> {
    if mails_content.len() == 0 {
        return vec![];
    }
    let mut mails: Vec<&str> = Vec::new();
    mails_content.split(MBOX_MAIL_SEPARATOR).for_each(|mail| {
        mails.push(mail);
    });
    mails
}

#[cfg(test)]
mod tests {
    use std::{fs, io};
    use super::*;

    #[test]
    fn empty_content_test() {
        let mails: Vec<&str> = split("");
        assert_eq!(mails.len(), 0);
    }

    #[test]
    fn one_mail_test() -> io::Result<()> {
        let mail_content = fs::read_to_string("tests/data/one_mail.mbox")?;
        let mails: Vec<&str> = split(&mail_content);
        assert_eq!(mails.len(), 1);
        assert_eq!(mails.get(0).unwrap(), &mail_content);
        Ok(())
    }

    #[test]
    fn two_mails_test() -> io::Result<()> {
        let mail_content = fs::read_to_string("tests/data/two_mails.mbox")?;
        let mails: Vec<&str> = split(&mail_content);
        assert_eq!(mails.len(), 2);
        Ok(())
    }
}

