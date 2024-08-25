// https://atcoder.jp/contests/abc295/tasks/abc295_c

use std::collections::HashMap;

fn run(_n: usize, a: Vec<usize>) -> usize {
    let mut hash_map = HashMap::new();

    for i in a.iter() {
        *hash_map.entry(i).or_insert(0) += 1;
    }

    hash_map.iter()
        .map(|(_, v)| v / 2)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        struct TestCase(usize, Vec<usize>, usize);

        let tests = [
            TestCase(6, vec![4, 1, 7, 4, 1, 4], 2),
            TestCase(1, vec![158260522], 0),
            TestCase(10, vec![295, 2, 29, 295, 29, 2, 29, 295, 2, 29], 4),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
