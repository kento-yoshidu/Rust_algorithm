// https://atcoder.jp/contests/abc092/tasks/abc092_b

fn run(_n: usize, d: usize, x: usize, a: Vec<usize>) -> usize {
    a.iter()
        .map(|i| {
            (d as f64 / *i as f64).ceil() as usize
        })
        .sum::<usize>() + x
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 7, 1, vec![2, 5, 10], 8),
            TestCase(2, 8, 20, vec![1, 10], 29),
            TestCase(5, 30, 44, vec![26, 18, 81, 18, 6], 56),
        ];

        for TestCase(n, d, x, a, expected) in tests {
            assert_eq!(run(n, d, x, a), expected);
        }
    }
}
