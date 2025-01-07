// https://atcoder.jp/contests/abc068/tasks/abc068_a

fn run(s: &str) -> String {
    String::from("ABC") + s
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("100", "ABC100"),
            TestCase("425", "ABC425"),
            TestCase("999", "ABC999"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
