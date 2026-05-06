// https://atcoder.jp/contests/abc451/tasks/abc451_a

fn run(s: &str) -> &'static str {
    if s.len() % 5 == 0 {
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
    fn abc451_a() {
        let tests = [
            TestCase("legal", "Yes"),
            TestCase("atcoder", "No"),
            TestCase("illegal", "No"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
