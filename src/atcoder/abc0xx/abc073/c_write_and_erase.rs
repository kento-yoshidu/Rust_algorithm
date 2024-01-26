// https://atcoder.jp/contests/abc073/tasks/abc073_c

use std::collections::HashMap;

pub fn run(_n: usize, a: Vec<usize>) -> usize {
    let mut hash_map = HashMap::new();

    for num in a {
        *hash_map.entry(num).or_insert(0) += 1;
    }

    hash_map.iter()
        .filter(|h| {
            h.1 % 2 != 0
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![6, 2, 6], 1),
            TestCase(4, vec![2, 5, 5, 2], 0),
            TestCase(6, vec![12, 22, 16, 22, 18, 12], 2),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(expected, run(n, a));
        }
    }
}
