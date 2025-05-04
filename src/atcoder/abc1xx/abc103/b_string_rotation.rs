// https://atcoder.jp/contests/abc103/tasks/abc103_b

fn run(s: &str, t: &str) -> &'static str {
    if (0..s.len()).any(|i| {
        s[i..].to_string() + &s[..i] == t
    }) {
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
    fn test() {
        let tests = [
            TestCase("kyoto", "tokyo", "Yes"),
            TestCase("abc", "arc", "No"),
            TestCase("aaaaaaaaaaaaaaab", "aaaaaaaaaaaaaaab", "Yes"),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
