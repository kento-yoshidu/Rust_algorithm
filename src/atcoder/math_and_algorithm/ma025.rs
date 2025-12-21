// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_y

fn run(_n: usize, a: Vec<usize>, b: Vec<usize>) -> f64 {
    a.into_iter()
        .zip(b)
        .fold(0.0, |acc, (a, b)| {
            acc + (a as f64 + b as f64 * 2.0) / 3.0
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>, f64);

    #[test]
    fn ma025() {
        let tests = [
            TestCase(5, vec![3, 1, 4, 1, 5], vec![9, 2, 6, 5, 3], 21.333333333333336),
        ];

        for TestCase(n, a, b, expected) in tests {
            assert_eq!(run(n, a, b), expected);
        }
    }
}
