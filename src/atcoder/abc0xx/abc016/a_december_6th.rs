// https://atcoder.jp/contests/abc016/tasks/abc016_1

fn run(m: usize, d: usize) -> &'static str {
    if m % d == 0 {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 1, "Yes"),
            TestCase(2, 29, "No"),
            TestCase(12, 6, "Yes"),
        ];

        for TestCase(m, d, expected) in tests {
            assert_eq!(run(m, d), expected);
        }
    }
}
