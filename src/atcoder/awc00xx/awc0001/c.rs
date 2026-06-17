// https://atcoder.jp/contests/awc0001/tasks/awc0001_c

use itertools::Itertools;

fn run(_n: usize, k: usize, d: Vec<usize>) -> usize {
    d.into_iter()
        .sorted()
        .rev()
        .skip(k)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn awc0001_c() {
        let tests = [
            TestCase(5, 2, vec![100, 250, 300, 150, 200], 450),
            TestCase(7, 3, vec![500, 1200, 800, 300, 950, 1100, 450], 2050),
            TestCase(10, 4, vec![1000000000, 999999999, 500000000, 750000000, 250000000, 800000000, 600000000, 400000000, 350000000, 900000000], 2850000000),
        ];

        for TestCase(n, k, d, expected) in tests {
            assert_eq!(run(n, k, d), expected);
        }
    }
}
