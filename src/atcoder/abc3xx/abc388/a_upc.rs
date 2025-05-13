// https://atcoder.jp/contests/abc388/tasks/abc388_a

fn run(s: &str) -> String {
    format!("{}UPC", s.chars().nth(0).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("Kyoto", "KUPC"),
            TestCase("Tohoku", "TUPC"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
