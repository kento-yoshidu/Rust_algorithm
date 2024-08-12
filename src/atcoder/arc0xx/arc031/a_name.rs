// https://atcoder.jp/contests/arc031/tasks/arc031_1

fn run(s: &str) -> &'static str {
    if s.chars().eq(s.chars().rev()) {
        "YES"
    } else {
        "NO"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("awawa", "YES"),
            TestCase("chokudai", "NO"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
