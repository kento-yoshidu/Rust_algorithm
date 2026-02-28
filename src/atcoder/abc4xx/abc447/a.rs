// https://atcoder.jp/contests/abc447/tasks/abc447_a

fn run(n: usize, m: usize) -> &'static str {
    if ((n + 1) / 2) >= m {
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
    fn abc447_a() {
        let tests = [
            TestCase(6, 3, "Yes"),
            TestCase(4, 3, "No"),
            TestCase(5, 3, "Yes"),
            TestCase(44, 7, "Yes"),
        ];

        for TestCase(n, m, expected) in tests {
            assert_eq!(run(n, m), expected);
        }
    }
}
