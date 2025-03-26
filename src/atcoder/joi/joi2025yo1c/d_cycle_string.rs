// https://atcoder.jp/contests/joi2025yo1c/tasks/joi2025_yo1c_d

fn run(n: usize, s: &str) -> &'static str {
    for i in 1..n {
        if n % i != 0 {
            continue;
        }

        let str = &s[0..i];

        if s == str.repeat(n/i) {
            return "Yes";
        }
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, "ababab", "Yes"),
            TestCase(7, "abcabca", "No"),
            TestCase(2, "aa", "Yes"),
            TestCase(8, "ababcdcd", "No"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
