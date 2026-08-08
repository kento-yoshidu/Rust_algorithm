// https://atcoder.jp/contests/awc0011/tasks/awc0011_a

use std::cmp::{min, max};

fn run(_n: usize, _m: usize, s: usize, t: usize, pv: Vec<(usize, usize)>) -> usize {
    pv.into_iter()
        .filter_map(|(p, v)| {
            let start = min(s, t);
            let end = max(s, t);

            (start <= p && p <= end).then_some(v)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, Vec<(usize, usize)>, usize);

    #[test]
    fn awc0011_a() {
        let tests = [
            TestCase(10, 5, 3, 8, vec![(1, 10), (3, 5), (5, 7), (8, 2), (10, 100)], 14),
            TestCase(100, 12, 40, 15, vec![(5, 12), (10, 7), (15, 100), (18, 30), (20, 8), (25, 50), (30, 3), (35, 20), (40, 200), (41, 9), (60, 15), (90, 1)], 411),
            TestCase(1000000000, 25, 999999920, 999999970, vec![(1, 111), (2, 222), (100000000, 333), (123456789, 444), (400000000, 555), (500000000, 666), (700000000, 777), (800000100, 888), (999999999, 999), (999999910, 4), (999999919, 70), (999999920, 100), (999999923, 1), (999999930, 500000000), (999999934, 20), (999999940, 300), (999999945, 999999999), (999999950, 42), (999999955, 17), (999999960, 250000000), (999999966, 6), (999999970, 8), (999999971, 90), (999999972, 3), (999999980, 5)], 1750000493),
        ];

        for TestCase(n, m, s, t, pv, expected) in tests {
            assert_eq!(run(n, m, s, t, pv), expected);
        }
    }
}
