// https://atcoder.jp/contests/abc145/tasks/abc145_b

fn run(n: usize, s: &str) -> &'static str {
    if n % 2 != 0 {
        return "No";
    }

    if &s[0..(n/2)] == &s[(n/2)..] {
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
    fn abc145_b() {
        let tests = [
            TestCase(6, "abcabc", "Yes"),
            TestCase(6, "abcadc", "No"),
            TestCase(1, "z", "No"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
