// https://atcoder.jp/contests/abc072/tasks/abc072_b

fn run(s: &str) -> String {
    s.chars().step_by(2).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("atcoder", "acdr"),
            TestCase("aaaa", "aa"),
            TestCase("z", "z"),
            TestCase("fukuokayamaguchi", "fkoaaauh"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
