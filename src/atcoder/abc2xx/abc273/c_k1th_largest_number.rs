// https://atcoder.jp/contests/abc273/tasks/abc273_c

use std::collections::BTreeMap;

fn run(n: usize, a: Vec<usize>) -> Vec<usize> {
    let mut tree_map: BTreeMap<usize, usize> = BTreeMap::new();

    for &ai in &a {
        *tree_map.entry(ai).or_insert(0) += 1;
    }

    let mut ans = Vec::new();

    for (_, v) in tree_map.iter().rev() {
        ans.push(*v);
    }

    for _ in 0..n - tree_map.len() {
        ans.push(0);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, vec![2, 7, 1, 8, 2, 8], vec![2, 1, 2, 1, 0, 0]),
            TestCase(1, vec![1], vec![1]),
            TestCase(10, vec![979861204, 57882493, 979861204, 447672230, 644706927, 710511029, 763027379, 710511029, 447672230, 136397527], vec![2, 1, 2, 1, 2, 1, 1, 0, 0, 0]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
