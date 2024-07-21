// https://atcoder.jp/contests/abc363/tasks/abc363_b

use itertools::Itertools;

pub fn run(_n: usize, t: usize, p: usize, l: Vec<usize>) -> usize {
    let vec: Vec<usize> =
        l.into_iter()
            .sorted()
            .rev()
            .collect();

    if vec[p-1] >= t {
        0
    } else {
        t - vec[p-1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 10, 3, vec![11, 3, 1, 6, 2], 7),
            TestCase(2, 5, 2, vec![10, 10], 0),
            TestCase(3, 10, 1, vec![1, 2, 3], 7),
        ];

        for TestCase(n, t, p, l, expected) in tests {
            assert_eq!(run(n, t, p, l), expected);
        }
    }
}
