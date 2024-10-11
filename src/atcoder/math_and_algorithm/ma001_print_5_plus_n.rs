// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_a

fn run(n: usize) -> usize {
    n + 5
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 7),
            TestCase(4, 9),
            TestCase(8, 13),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
