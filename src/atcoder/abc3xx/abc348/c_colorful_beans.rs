// https://atcoder.jp/contests/abc348/tasks/abc348_c

use std::cmp::min;
use std::collections::HashMap;

pub fn run(_n: usize, ac: Vec<(usize, usize)>) -> usize {
    let mut hash_map = HashMap::new();

    for (a, c) in ac.iter() {
        hash_map.insert(c, *min(a, hash_map.get(c).unwrap_or(&std::usize::MAX)));
    }

    hash_map.into_iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![(100, 1), (20, 5), (30, 5), (40, 1)], 40),
            TestCase(10, vec![(68, 3), (17, 2), (99, 2), (92, 4), (82, 4), (10, 3), (100, 2), (78, 1), (3, 1), (35, 4)], 35),
        ];

        for TestCase(n, ac, expected) in tests {
            assert_eq!(run(n, ac), expected);
        }
    }
}
