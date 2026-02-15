// https://atcoder.jp/contests/abc251/tasks/abc251_a

fn run(s: &str) -> String {
    s.repeat(6 / s.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc251_a() {
        let tests = [
            TestCase("ww", "wwwwww"),
            TestCase("abc", "abcabc"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
