// https://atcoder.jp/contests/abc374/tasks/abc374_a

fn run(s: &str) -> &'static str {
    if &s[s.len()-3..] == "san" {
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
            TestCase("takahashisan", "Yes"),
            TestCase("aokikun", "No"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
