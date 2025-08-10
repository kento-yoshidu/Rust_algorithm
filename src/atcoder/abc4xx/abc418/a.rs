// https://atcoder.jp/contests/abc418/tasks/abc418_a

fn run(n: usize, s: &str) -> &'static str {
    if n < 3 {
        return "No";
    }

    if &s[n-3..] == "tea" {
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
    fn abc418_a() {
        let tests = [
            TestCase(8, "greentea", "Yes"),
            TestCase(6, "coffee", "No"),
            TestCase(3, "tea", "Yes"),
            TestCase(1, "t", "No"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
