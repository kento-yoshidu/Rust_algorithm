// https://atcoder.jp/contests/abc193/tasks/abc193_a

fn run(a: usize, b: usize) -> f64 {
    ((a - b) as f64 / a as f64) * 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, f64);

    #[test]
    fn abc193_a() {
        let tests = [
            TestCase(100, 80, 20.0 ),
            TestCase(7, 6, 14.285714285714285),
            TestCase(99999, 99998, 0.00100001000010000100),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
