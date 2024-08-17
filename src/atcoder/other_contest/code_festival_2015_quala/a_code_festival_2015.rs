// https://atcoder.jp/contests/code-festival-2015-quala/tasks/codefestival_2015_qualA_a

fn run(s: &str) -> String {
    format!("{}2015", &s[..s.len()-4])
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("CODEFESTIVAL2014", "CODEFESTIVAL2015"),
            TestCase("CHOKUDAI2014", "CHOKUDAI2015"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}