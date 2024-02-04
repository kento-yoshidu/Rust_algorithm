// https://atcoder.jp/contests/abc339/tasks/abc339_a

pub fn run<'a>(s: &'a str) -> &'a str {
    &s.split(".").last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("atcoder.jp", "jp"),
            TestCase("translate.google.com", "com"),
            TestCase(".z", "z"),
            TestCase("..........txt", "txt"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
