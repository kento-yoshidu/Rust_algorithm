// https://atcoder.jp/contests/code-festival-2017-qualb/tasks/code_festival_2017_qualb_a

fn run(s: &str) -> &str {
    &s[..s.len()-8]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("CODEFESTIVAL", "CODE"),
            TestCase("CODEFESTIVALFESTIVAL", "CODEFESTIVAL"),
            TestCase("YAKINIKUFESTIVAL", "YAKINIKU"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
