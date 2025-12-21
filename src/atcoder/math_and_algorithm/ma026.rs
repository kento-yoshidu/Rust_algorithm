// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_z

fn run(n: usize) -> f64 {
    (1..=n)
        .map(|i| 1.0 * n as f64 / i as f64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, f64);

    #[test]
    fn ma026() {
        let tests = [
            TestCase(5, 11.416666666666666),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
