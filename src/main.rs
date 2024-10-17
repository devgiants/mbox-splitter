fn main() {}

fn split(_mails_content: &str) -> Vec<&str> {
    vec![]
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
        Ok(())
    }
}

