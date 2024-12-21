// https://atcoder.jp/contests/joi2023yo1b/tasks/joi2023_yo1b_c

fn run(n: usize, s: &str) -> &'static str {
    if s[0..n/2] == s[n/2..] {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, "JOIJOI", "Yes"),
            TestCase(6, "IOIOIO", "No"),
            TestCase(2, "OO", "Yes"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
