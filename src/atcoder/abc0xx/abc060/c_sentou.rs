// https://atcoder.jp/contests/abc060/tasks/arc073_a

use std::cmp::min;

pub fn run(_n: usize, t: usize, vec: Vec<usize>) -> usize {
    vec.windows(2)
        .map(|v| {
            min(t, v[1] - v[0])
        })
        .sum::<usize>() + t
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 4, vec![0, 3], 7),
            TestCase(2, 4, vec![0, 5], 8),
            TestCase(4, 1000000000, vec![0, 1000, 1000000, 1000000000], 2000000000),
            TestCase(1, 1, vec![1], 1),
            TestCase(9, 10, vec![0, 3, 5, 7, 100, 110, 200, 300, 311], 67),
        ];

        for TestCase(n, t, vec, expected) in tests {
            assert_eq!(expected, run(n, t, vec));
        }
    }
}
