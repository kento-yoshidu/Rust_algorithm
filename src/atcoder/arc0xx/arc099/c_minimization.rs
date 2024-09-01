// https://atcoder.jp/contests/abc101/tasks/arc099_a

fn run(n: usize, k: usize, _a: Vec<usize>) -> usize {
    ((n-1) as f64 / (k-1) as f64).ceil() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 3, vec![2, 3, 1, 4], 2),
            TestCase(3, 3, vec![1, 2, 3], 1),
            TestCase(8, 3, vec![7, 3, 1, 8, 4, 6, 2, 5], 4),
        ];

        for TestCase(n, k, a, expected) in tests {
            assert_eq!(run(n, k, a), expected);
        }
    }
}
