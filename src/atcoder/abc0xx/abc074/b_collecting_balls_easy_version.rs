// https://atcoder.jp/contests/abc074/tasks/abc074_b

fn run(_n: usize, k: usize, x: Vec<usize>) -> usize {
    x.iter()
        .map(|i| {
            (i - 0).min(k - i) * 2
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 10, vec![2], 4),
            TestCase(2, 9, vec![3, 6], 12),
            TestCase(5, 20, vec![11, 12, 9, 17, 12], 74),
        ];

        for TestCase(n, k, x, expected) in tests {
            assert_eq!(run(n, k, x), expected);
        }
    }
}
