// https://atcoder.jp/contests/abc303/tasks/abc303_a

fn run(_n: usize, s: &'static str, t: &'static str) -> &'static str {
    if s.replace("0", "o").replace("1", "l") == t.replace("0", "o").replace("1", "l") {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, "l0w", "1ow", "Yes"),
            TestCase(3, "abc", "abr", "No"),
            TestCase(4, "nok0", "n0ko", "Yes"),
        ];

        for TestCase(n, s, t, expected) in tests {
            assert_eq!(run(n, s, t), expected);
        }
    }
}
