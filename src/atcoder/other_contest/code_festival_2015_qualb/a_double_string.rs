// https://atcoder.jp/contests/code-festival-2015-qualb/tasks/codefestival_2015_qualB_a

fn run(s: &str) -> String {
    format!("{}{}", s, s)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("no", "nono"),
            TestCase("meat", "meatmeat"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
