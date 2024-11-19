// https://atcoder.jp/contests/abc028/tasks/abc028_a

fn run(n: usize) -> &'static str {
    if n == 100 {
        "Perfect"
    } else if n >= 90 {
        "Great"
    } else if n >= 60 {
        "Good"
    } else {
        "Bad"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(80, "Good"),
            TestCase(100, "Perfect"),
            TestCase(59, "Bad"),
            TestCase(95, "Great"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
