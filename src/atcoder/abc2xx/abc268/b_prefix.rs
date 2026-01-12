// https://atcoder.jp/contests/abc268/tasks/abc268_b

fn run(s: &str, t: &str) -> &'static str {
    if s.len() > t.len() {
        return "No";
    }

    if s.chars()
        .zip(t.chars())
        .all(|(l, r)| {
            l == r
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
    fn abc268_b() {
        let tests = [
            TestCase("atco", "atcoder", "Yes"),
            TestCase("code", "atcoder", "No"),
            TestCase("abc", "abc", "Yes"),
            TestCase("aaaa", "aa", "No"),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
