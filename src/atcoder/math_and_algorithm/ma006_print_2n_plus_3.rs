// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_f

fn run(n: usize) -> usize {
    2 * n + 3
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(100, 203),
            TestCase(27, 57),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
