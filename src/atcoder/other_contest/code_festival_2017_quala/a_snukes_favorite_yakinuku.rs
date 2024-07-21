// https://atcoder.jp/contests/code-festival-2017-quala/tasks/code_festival_2017_quala_a

fn run(s: &str) -> &'static str {
    if s.get(..4).unwrap_or(s) == "YAKI" {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("YAKINIKU", "Yes"),
            TestCase("TAKOYAKI", "No"),
            TestCase("YAK", "No")
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
