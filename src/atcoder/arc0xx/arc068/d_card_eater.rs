// https://atcoder.jp/contests/abc053/tasks/arc068_b

use std::collections::HashMap;

fn run(_n: usize, a: Vec<usize>) -> usize {
    let mut hash_map = HashMap::new();

    for n in a {
        *hash_map.entry(n).or_insert(0) += 1;
    }

    let count = hash_map.iter()
        .filter(|(_, v)| **v % 2 == 0)
        .count();

    if count % 2 == 0 {
        hash_map.len()
    } else {
        hash_map.len()-1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, vec![1, 2, 1, 3, 7], 3),
            TestCase(15, vec![1, 3, 5, 2, 1, 3, 2, 8, 8, 6, 2, 6, 11, 1, 1], 7),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
