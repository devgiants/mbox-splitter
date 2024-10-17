fn main() {

}

fn split(_mails_content: &str) -> Vec<&str> {
    vec![]
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

