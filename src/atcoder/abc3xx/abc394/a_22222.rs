// https://atcoder.jp/contests/abc394/tasks/abc394_a

fn run(s: &str) -> String {
    s.chars()
        .filter(|c| *c == '2')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("20250222", "22222"),
            TestCase("2", "2"),
            TestCase("22222000111222222", "22222222222"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
