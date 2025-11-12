// https://atcoder.jp/contests/abc205/tasks/abc205_a

fn run(a: usize, b: usize) -> f64 {
    a as f64 * (b as f64 / 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, f64);

    #[test]
    fn abc205_a() {
        let tests = [
            TestCase(45, 200, 90.0),
            TestCase(37, 450, 166.5),
            TestCase(0, 1000, 0.0),
            TestCase(50, 0, 0.0),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
