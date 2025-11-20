// https://atcoder.jp/contests/abc217/tasks/abc217_a

fn run(s: &str, t: &str) -> &'static str {
    if s < t {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, &'static str);

    #[test]
    fn abc217_a() {
        let tests = [
            TestCase("abc", "atcoder", "Yes"),
            TestCase("arc", "agc", "No"),
            TestCase("a", "aa", "Yes"),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
