// https://atcoder.jp/contests/abc169/tasks/abc169_c

fn run(a: usize, b: f64) -> usize {
    (a * (b * 100.0).round() as usize) / 100
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, f64, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(198, 1.10, 217),
            TestCase(1, 0.01, 0),
            TestCase(1000000000000000, 9.99, 9990000000000000),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
