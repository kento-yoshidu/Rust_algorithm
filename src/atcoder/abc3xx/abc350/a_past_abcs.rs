pub fn run(s: &str) -> &'static str {
    let number = s[3..].parse::<usize>().unwrap();

    if number >= 350 || number == 316 {
        "No"
    } else {
        "Yes"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("ABC349", "Yes"),
            TestCase("ABC350", "No"),
            TestCase("ABC316", "No"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
