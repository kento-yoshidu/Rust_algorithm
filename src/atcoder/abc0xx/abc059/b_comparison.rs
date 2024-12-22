// https://atcoder.jp/contests/abc059/tasks/abc059_b

fn run(a: &str, b: &str) -> &'static str {
    if a.len() > b.len() {
        "GREATER"
    } else if a.len() < b.len() {
        "LESS"
    } else if a == b {
        "EQUAL"
    } else if a > b {
        "GREATER"
    } else {
        "LESS"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("36", "24", "GREATER"),
            TestCase("850", "3777", "LESS"),
            TestCase("9720246", "22516266", "LESS"),
            TestCase("123456789012345678901234567890", "234567890123456789012345678901", "LESS"),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
