// https://atcoder.jp/contests/abc226/tasks/abc226_b

use std::collections::HashSet;

fn run(_n: usize, a: Vec<Vec<usize>>) -> usize {
    let mut hash_set = HashSet::new();

    for vec in a.iter() {
        hash_set.insert(&vec[1..]);
    }

    hash_set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<Vec<usize>>, usize);

    #[test]
    fn abc226_b() {
        let tests = [
            TestCase(4, vec![vec![2, 1, 2], vec![2, 1, 1], vec![2, 2, 1], vec![2, 1, 2]], 3),
            TestCase(5, vec![vec![1, 1], vec![1, 1], vec![1, 2], vec![2, 1, 1], vec![3, 1, 1, 1]], 4),
            TestCase(1, vec![vec![1, 1]], 1),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
