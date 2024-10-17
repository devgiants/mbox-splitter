fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_content_test() {
        let mails: Vec<&str> = split("");
        assert_eq!(mails.len(), 0);
    }
}

