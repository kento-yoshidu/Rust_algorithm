// https://atcoder.jp/contests/joi2020yo1a/tasks/joi2020_yo1a_c

use itertools::Itertools;

fn run(_n: usize, _m: usize, a: Vec<usize>, b: Vec<usize>) -> Vec<usize> {
    a.into_iter()
        .chain(b.into_iter())
        .sorted()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 1, vec![1, 2], vec![2], vec![1, 2, 2]),
            TestCase(3, 8, vec![1, 3, 8], vec![3, 3, 4, 5, 6, 7, 8, 9], vec![1, 3, 3, 3, 4, 5, 6, 7, 8, 8, 9]),
        ];

        for TestCase(n, m, a, b, expected) in tests {
            assert_eq!(run(n, m, a, b), expected);
        }
    }
}
