// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_e

fn run(_n: usize, a: Vec<usize>) -> usize {
    a.into_iter().sum::<usize>() % 100
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![30, 50, 70], 50),
            TestCase(10, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 55),
            TestCase(5, vec![60, 60, 60, 60, 60], 0),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
