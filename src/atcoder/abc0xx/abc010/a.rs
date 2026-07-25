// https://atcoder.jp/contests/abc010/tasks/abc010_1

fn run(s: &str) -> String {
    s.to_string() + "pp"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc010_a() {
        let tests = [
            TestCase("chokudai", "chokudaipp"),
            TestCase("sanagi", "sanagipp"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
