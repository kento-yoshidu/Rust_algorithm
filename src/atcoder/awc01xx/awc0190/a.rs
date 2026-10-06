// https://atcoder.jp/contests/awc0119/tasks/awc0119_a

use std::collections::HashSet;

fn run(_n: usize, s: Vec<usize>) -> (usize, usize) {
    let mut set = HashSet::new();

    for s in s {
        set.insert(s);
    }

    (set.len(), set.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, (usize, usize));

    #[test]
    fn awc0190_a() {
        let tests = [
            TestCase(5, vec![3, 1, 4, 1, 5], (4, 13)),
            TestCase(10, vec![7, 2, 7, 3, 2, 8, 3, 7, 1, 4], (6, 25)),
            TestCase(20, vec![50, 25, 75, 25, 100, 50, 30, 75, 25, 100, 1, 30, 50, 99, 1, 75, 42, 99, 42, 88], (9, 510)),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
