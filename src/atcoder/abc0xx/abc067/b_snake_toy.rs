// https://atcoder.jp/contests/abc067/tasks/abc067_a

use itertools::Itertools;

fn run(_n: usize, k: usize, vec: Vec<usize>) -> usize {
    vec.into_iter()
        .sorted()
        .rev()
        .take(k)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct  TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 3, vec![1, 2, 3, 4, 5], 12),
            TestCase(15, 14, vec![50, 26, 27, 21, 41, 7, 42, 35, 7, 5, 5, 36, 39, 1, 45], 386),
        ];

        for TestCase(n, k, vec, expected) in tests {
            assert_eq!(run(n, k, vec), expected);
        }
    }
}
