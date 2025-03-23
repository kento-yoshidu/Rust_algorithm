// https://atcoder.jp/contests/arc015/tasks/arc015_1

fn run(n: isize) -> f64 {
    9.0 / 5.0 * n as f64 + 32.0
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, f64);

    #[test]
    fn test() {
        let tests = [
            TestCase(10, 50.0),
            TestCase(33, 91.4),
            TestCase(-100, -148.0),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
