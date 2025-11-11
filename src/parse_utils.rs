pub fn parse_numbers(input: &str) -> Vec<u64> {
    let re = regex::Regex::new(r"\d+").expect("Failed to compile regex");
    re.find_iter(input)
        .map(|m| m.as_str().parse().unwrap())
        .collect()
}

pub fn parse_signed_numbers(input: &str) -> Vec<i64> {
    let re = regex::Regex::new(r"-?\d+").expect("Failed to compile regex");
    re.find_iter(input)
        .map(|m| m.as_str().parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numbers() {
        assert_eq!(parse_numbers("123 456 789"), vec![123, 456, 789]);
        assert_eq!(parse_numbers("abc123def456ghi"), vec![123, 456]);
        assert_eq!(parse_numbers("no numbers here"), vec![]);
        assert_eq!(parse_numbers("42"), vec![42]);
        assert_eq!(parse_numbers("1a2b3c"), vec![1, 2, 3]);
    }

    #[test]
    fn test_parse_signed_numbers() {
        assert_eq!(parse_signed_numbers("123 -456 789"), vec![123, -456, 789]);
        assert_eq!(parse_signed_numbers("abc-123def456ghi"), vec![-123, 456]);
        assert_eq!(parse_signed_numbers("no numbers here"), vec![]);
        assert_eq!(parse_signed_numbers("-42"), vec![-42]);
        assert_eq!(parse_signed_numbers("1a-2b3c"), vec![1, -2, 3]);
        // assert_eq!(parse_signed_numbers("--5"), vec![5]);
    }
}
