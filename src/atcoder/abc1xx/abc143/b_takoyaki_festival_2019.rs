// https://atcoder.jp/contests/abc143/tasks/abc143_b

use itertools::Itertools;

fn run(_n: usize, d: Vec<usize>) -> usize {
    d.into_iter()
        .tuple_combinations()
        .map(|(a, b)| a * b)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![3, 1, 2], 11),
            TestCase(7, vec![5, 0, 7, 8, 3, 3, 2], 312),
        ];

        for TestCase(n, d, expected) in tests {
            assert_eq!(run(n, d), expected);
        }
    }
}
