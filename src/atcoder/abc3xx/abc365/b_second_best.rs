// https://atcoder.jp/contests/abc365/tasks/abc365_b

use itertools::Itertools;

fn run(_n: usize, a: Vec<usize>) -> usize {
    a.into_iter()
        .enumerate()
        .sorted_by(|a, b| b.1.cmp(&a.1))
        .nth(1)
        .map(|(i, _)| i+1)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![8, 2, 5, 1], 3),
            TestCase(8, vec![1, 2, 3, 4, 5, 10, 9, 11], 6),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
