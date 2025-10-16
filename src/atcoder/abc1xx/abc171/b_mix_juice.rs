// https://atcoder.jp/contests/abc171/tasks/abc171_b

use itertools::Itertools;

fn run(_n: usize, k: usize, p: Vec<usize>) -> usize {
    let vec: Vec<usize> = p.into_iter().sorted().collect();

    (0..k)
        .map(|i| {
            vec[i]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn abc171_b() {
        let tests = [
            TestCase(5, 3, vec![50, 100, 80, 120, 80], 210),
            TestCase(1, 1, vec![1000], 1000),
        ];

        for TestCase(n, k, p, expected) in tests {
            assert_eq!(run(n, k, p), expected);
        }
    }
}
