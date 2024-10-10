// https://atcoder.jp/contests/abc367/tasks/abc367_b

fn run(x: &str) -> String {
    let mut s = x.trim_end_matches('0');
    s = s.trim_end_matches('.');

    s.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("1.012", "1.012"),
            TestCase("12.340", "12.34"),
            TestCase("99.900", "99.9"),
            TestCase("0.000", "0"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
