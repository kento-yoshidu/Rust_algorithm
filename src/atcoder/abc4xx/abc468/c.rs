// https://atcoder.jp/contests/abc468/tasks/abc468_c

use itertools::Itertools;

fn run(n: usize, p: Vec<usize>, q: Vec<usize>) -> usize {
    (1..=n)
        .permutations(n)
        .filter(|r| p < *r && *r < q)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>, usize);

    #[test]
    fn abc468_c() {
        let tests = [
            TestCase(3, vec![1, 3, 2], vec![3, 1, 2], 2),
            TestCase(5, vec![5, 4, 2, 1, 3], vec![5, 1, 2, 3, 4], 0),
            TestCase(7, vec![3, 6, 5, 2, 7, 1, 4], vec![4, 1, 5, 7, 2, 3, 6], 223),
        ];

        for TestCase(n, p, q, expected) in tests {
            assert_eq!(run(n, p, q), expected);
        }
    }
}
