// https://atcoder.jp/contests/abc088/tasks/abc088_a

fn run(n: usize, a: usize) -> &'static str {
    if n % 500 <= a {
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
            TestCase(2018, 218, "Yes"),
            TestCase(2763, 0, "No"),
            TestCase(37, 514, "Yes"),
            TestCase(37, 37, "Yes"),
            TestCase(37, 36, "No"),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
