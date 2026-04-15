// https://atcoder.jp/contests/abc445/tasks/abc445_a

fn run(s: &str) -> &'static str {
    if s.chars().nth(0).unwrap() == s.chars().nth(s.len() - 1).unwrap() {
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
    fn abc445_a() {
        let tests = [
            TestCase("luminol", "Yes"),
            TestCase("rule", "No"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
