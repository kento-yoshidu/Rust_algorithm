// https://atcoder.jp/contests/abc167/tasks/abc167_a

fn run(s: &str, t: &str) -> &'static str {
    if s == &t[0..t.len()-1] {
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
    fn abc167_a() {
        let tests = [
            TestCase("chokudai", "chokudaiz", "Yes"),
            TestCase("snuke", "snekee", "No"),
            TestCase("a", "aa", "Yes"),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
