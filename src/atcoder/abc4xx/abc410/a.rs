// https://atcoder.jp/contests/abc410/tasks/abc410_a

fn run(_n: usize, a: Vec<usize>, k: usize) -> usize {
    a.into_iter()
        .filter(|n| *n >= k)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize, usize);

    #[test]
    fn abc410_b() {
        let tests = [
            TestCase(5, vec![3, 1, 4, 1, 5], 4, 2),
            TestCase(1, vec![1], 100, 0),
            TestCase(15, vec![18, 89, 31, 2, 15, 93, 64, 78, 58, 19, 79, 59, 24, 50, 30], 38, 8),
        ];

        for TestCase(n, a, k, expected) in tests {
            assert_eq!(run(n, a, k), expected);
        }
    }
}
