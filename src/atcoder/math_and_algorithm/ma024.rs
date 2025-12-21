// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_x

fn run(_n: usize, pq: Vec<(usize, usize)>) -> f64 {
    pq.into_iter()
        .fold(0.0, |acc, (x, y)| {
            acc + y as f64 / x as f64
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, f64);

    #[test]
    fn ma024() {
        let tests = [
            TestCase(2, vec![(2, 50), (4, 100)], 50.0),
        ];

        for TestCase(n, pq, expected) in tests {
            assert_eq!(run(n, pq), expected);
        }
    }
}
