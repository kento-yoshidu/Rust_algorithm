// https://atcoder.jp/contests/abc443/tasks/abc443_a

fn run(s: &str) -> String {
    format!("{}s", s)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc443_a() {
        let tests = [
            TestCase("http", "https"),
            TestCase("append", "appends"),
            TestCase("beginner", "beginners"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
