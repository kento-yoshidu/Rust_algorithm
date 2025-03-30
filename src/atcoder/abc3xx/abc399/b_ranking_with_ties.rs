// https://atcoder.jp/contests/abc399/tasks/abc399_b

use itertools::Itertools;

fn run(_n: usize, p: Vec<usize>) -> Vec<usize> {
    let rank: Vec<&usize> = p.iter().sorted().rev().collect();

    p.iter()
        .map(|n| {
            rank.iter().position(|&x| x == n).unwrap() + 1
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![3, 12, 9, 9], vec![4, 1, 2, 2]),
            TestCase(3, vec![3, 9, 6], vec![3, 1, 2]),
            TestCase(4, vec![100, 100, 100, 100], vec![1, 1, 1, 1]),
            TestCase(8, vec![87, 87, 87, 88, 41, 38, 41, 38], vec![2, 2, 2, 1, 5, 7, 5, 7]),
        ];

        for TestCase(n, p, expected) in tests {
            assert_eq!(run(n, p), expected);
        }
    }
}
