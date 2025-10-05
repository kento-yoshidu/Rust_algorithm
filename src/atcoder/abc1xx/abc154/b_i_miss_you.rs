// https://atcoder.jp/contests/abc154/tasks/abc154_b

fn run(s: &str) -> String {
    format!("{}", "x".repeat(s.len()))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc154_b() {
        let tests = [
            TestCase("sardine", "xxxxxxx"),
            TestCase("xxxx", "xxxx"),
            TestCase("gone", "xxxx"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
