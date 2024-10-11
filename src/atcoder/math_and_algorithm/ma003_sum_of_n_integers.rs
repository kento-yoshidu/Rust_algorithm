// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_c

fn run(_n: usize, a: Vec<usize>) -> usize {
    a.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, vec![3, 1, 4, 1, 5], 14),
            TestCase(3, vec![10, 20, 50], 80),
            TestCase(10, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 55),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
