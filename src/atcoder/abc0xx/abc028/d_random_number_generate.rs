// https://atcoder.jp/contests/abc028/tasks/abc028_d

fn run(n: f64, k: f64) -> f64 {
    ((n - k) * (k - 1.0) * 6.0 + (n - 1.0) * 3.0 + 1.0) / n.powf(3.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(f64, f64, f64);

    #[test]
    fn test() {
        let tests = [
            TestCase(3.0, 2.0, 0.48148148148148145),
            TestCase(3.0, 1.0, 0.25925925925925924),
            TestCase(765.0, 573.0, 0.0014769739698462438),
        ];

        for TestCase(n, k, expected) in tests {
            assert_eq!(run(n, k), expected);
        }
    }
}
