// https://atcoder.jp/contests/abc010/tasks/abc010_1

fn run(s: String) -> String {
    s + "pp"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("chokudai", "chokudaipp"),
            TestCase("sanagi", "sanagipp"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s.to_string()), expected);
        }
    }
}
