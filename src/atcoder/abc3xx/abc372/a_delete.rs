// https://atcoer.jp/contests/abc372/tasks/abc372_a

fn run(s: &str) -> String {
    s.chars()
        .filter(|c| *c != '.')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(".v.", "v"),
            TestCase("chokudai", "chokudai"),
            TestCase("...", ""),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
