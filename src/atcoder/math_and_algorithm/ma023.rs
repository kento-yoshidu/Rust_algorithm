// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_w

fn run(n: usize, b: Vec<usize>, r: Vec<usize>) -> f64 {
    b.into_iter()
        .zip(r)
        .map(|(b, r)| b as f64 / n as f64 + r as f64 / n as f64)
        .sum::<f64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>, f64);

    #[test]
    fn ma023() {
        let tests = [
            TestCase(3, vec![1, 2, 3], vec![10, 20, 30], 22.00),
        ];

        for TestCase(n, b, r, expected) in tests {
            assert_eq!(run(n, b, r), expected);
        }
    }
}
