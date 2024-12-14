// https://atcoder.jp/contests/joi2020yo1b/tasks/joi2020_yo1b_c

use std::collections::HashMap;

fn run(_n: usize, _m: usize, a: Vec<usize>) -> usize {
    let mut hash_map = HashMap::new();

    for n in a {
        *hash_map.entry(n).or_insert(0) += 1;
    }

    hash_map.
        into_iter()
        .max_by(|a, b| a.1.cmp(&(b.1)))
        .unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 3, vec![1, 1, 2, 3], 2),
            TestCase(6, 5, vec![3, 3, 2, 1, 2, 3], 3),
        ];

        for TestCase(n, m, a, expected) in tests {
            assert_eq!(run(n, m, a), expected);
        }
    }
}
