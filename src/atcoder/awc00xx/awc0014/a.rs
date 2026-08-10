// https://atcoder.jp/contests/awc0014/tasks/awc0014_a

use std::cmp::min;

fn run(_n: usize, r: usize, t: usize, p: Vec<usize>) -> Vec<usize> {
    p.into_iter()
        .map(|p| {
            min(r, t / p)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, Vec<usize>);

    #[test]
    fn awc0014_a() {
        let tests = [
            TestCase(3, 5, 20, vec![3, 5, 7], vec![5, 4, 2]),
            TestCase(4, 10, 15, vec![1, 4, 8, 16], vec![10, 3, 1, 0]),
            TestCase(10, 100, 1000, vec![1, 2, 5, 10, 50, 100, 200, 333, 500, 1001], vec![100, 100, 100, 100, 20, 10, 5, 3, 2, 0]),
            TestCase(20, 1000000000, 1000000000000000000, vec![1, 2, 3, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000, 999999999, 500000000, 250000000, 123456789, 777777777, 42, 314159265, 271828182], vec![1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000]),
            TestCase(1, 1000000000, 1, vec![1000000000], vec![0]),
        ];

        for TestCase(n, r, t, p, expected) in tests {
            assert_eq!(run(n, r, t, p), expected);
        }
    }
}
