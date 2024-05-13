// https://atcoder.jp/contests/tenka1-2017-beginner/tasks/tenka1_2017_b

use itertools::Itertools;

pub fn run(_n: usize, ab: Vec<(usize, usize)>) -> usize {
    let min = ab.iter()
        .map(|(_, b)| b)
        .position_min()
        .unwrap();

    ab[min].0 + ab[min].1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![(4, 7), (2, 9), (6, 2)], 8),
            TestCase(5, vec![(1, 10), (3, 6), (5, 2), (4, 4), (2, 8)], 7),
            TestCase(2, vec![(1, 1000000000), (1000000000, 1)], 1000000001),
        ];

        for TestCase(n, ab, expected) in tests {
            assert_eq!(run(n, ab), expected);
        }
    }
}
