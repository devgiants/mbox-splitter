fn main() {}

fn split(mails_content: &str) -> Vec<&str> {
    let mut mails: Vec<&str> = Vec::new();
    if mails_content.len() > 0 {
        mails.push(mails_content);
    }
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
}

