// https://atcoder.jp/contests/awc0027/tasks/awc0027_a

use std::cmp::{min, max};

fn run(_n: usize, s: usize, t: usize, a: Vec<usize>) -> usize {
    a.into_iter()
        .filter(|a| {
            max(*a, s) - min(*a, s) <= t
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, usize);

    #[test]
    fn awc0027_a() {
        let tests = [
            TestCase(5, 60, 10, vec![55, 70, 62, 45, 58], 4),
            TestCase(8, 75, 5, vec![80, 75, 70, 68, 82, 74, 76, 90], 5),
            TestCase(15, 50, 0, vec![50, 49, 51, 50, 48, 52, 50, 50, 47, 53, 50, 46, 54, 50, 55], 6),
        ];

        for TestCase(n, s, t, a, expected) in tests {
            assert_eq!(run(n, s, t, a), expected);
        }
    }
}
