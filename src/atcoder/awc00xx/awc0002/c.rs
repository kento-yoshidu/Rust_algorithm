// https://atcoder.jp/contests/awc0002/tasks/awc0002_c

use std::cmp::max;

fn run(_n: usize, m: isize, ab: Vec<(isize, isize)>) -> usize {
    ab.into_iter()
        .map(|(a, b)| {
            let rest = max(0, m-a);
            (rest + b - 1) / b
        })
        .max()
        .unwrap() as usize
} 

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, isize, Vec<(isize, isize)>, usize);

    #[test]
    fn awc0002_c() {
        let tests = [
            TestCase(3, 10, vec![(2, 3), (5, 2), (8, 1)], 3),
            TestCase(4, 100, vec![(100, 5), (90, 10), (95, 3), (80, 4)], 5),
            TestCase(5, 1000000000, vec![(1, 1), (500000000, 100000000), (999999999, 1), (100000000, 300000000), (250000000, 250000000)], 999999999),
        ];

        for TestCase(n, m, ab, expected) in tests {
            assert_eq!(run(n, m, ab), expected);
        }
    }
}
