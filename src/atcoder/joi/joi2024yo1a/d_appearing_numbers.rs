// https://atcoder.jp/contests/joi2024yo1a/tasks/joi2024_yo1a_d

use std::collections::BTreeSet;

fn run(_n: usize, a: Vec<usize>) -> Vec<usize> {
    let mut vec = a.clone();

    vec.sort();
    vec.dedup();

    vec
}

pub fn run2(_n: usize, a: Vec<usize>) -> Vec<usize> {
    let mut btree_set = BTreeSet::new();

    for i in a {
        btree_set.insert(i);
    }

    btree_set.iter().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(8, vec![2, 0, 2, 3, 0, 9, 1, 6], vec![0, 1, 2, 3, 6, 9]),
            TestCase(3, vec![9, 9, 9], vec![9]),
            TestCase(10, vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3], vec![1, 2, 3, 4, 5, 6, 9]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
