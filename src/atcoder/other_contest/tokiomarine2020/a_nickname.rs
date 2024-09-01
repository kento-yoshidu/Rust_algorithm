// https://atcoder.jp/contests/tokiomarine2020/tasks/tokiomarine2020_a

fn run(s: &str) -> &str {
    &s[0..3]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("takahashi", "tak"),
            TestCase("naohiro", "nao"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
