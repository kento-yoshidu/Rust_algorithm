// https://atcoder.jp/contests/abc238/tasks/abc238_a

fn run(n: usize) -> &'static str {
    if 2 <= n && n <= 4 {
        "No"
    } else {
        "Yes"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn abc238_a() {
        let tests = [
            TestCase(5, "Yes"),
            TestCase(2, "No"),
            TestCase(623947744, "Yes"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
