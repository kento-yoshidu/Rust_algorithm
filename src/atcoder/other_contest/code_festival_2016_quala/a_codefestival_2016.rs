// https://atcoder.jp/contests/code-festival-2016-quala/tasks/codefestival_2016_qualA_a

fn run(s: &str) -> String {
    format!("{} {}", &s[0..4], &s[4..])
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("CODEFESTIVAL", "CODE FESTIVAL"),
            TestCase("POSTGRADUATE", "POST GRADUATE"),
            TestCase("ABCDEFGHIJKL", "ABCD EFGHIJKL"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
