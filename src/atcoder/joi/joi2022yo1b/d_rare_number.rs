// https://atcoder.jp/contests/joi2022yo1b/tasks/joi2022_yo1b_d

use std::collections::BTreeMap;

fn run(_n: usize, a: Vec<usize>) -> usize {
    let mut btree_map = BTreeMap::new();

    for n in a {
        *btree_map.entry(n).or_insert(0) += 1;
    }

    btree_map.into_iter()
        .min_by_key(|&(_, count)| count)
        .map(|(k, _)| k)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![3, 4, 3], 4),
            TestCase(5, vec![4, 4, 8, 2, 5], 2),
            TestCase(8, vec![8, 8, 7, 7, 6, 6, 5, 5], 5),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
