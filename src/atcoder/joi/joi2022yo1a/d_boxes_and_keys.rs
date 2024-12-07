// https://atcoder.jp/contests/joi2022yo1a/tasks/joi2022_yo1a_d

use std::collections::{HashMap, HashSet};

fn run(_n: usize, _m: usize, a: Vec<usize>, b: Vec<usize>) -> usize {
    let mut map_a = HashMap::new();
    let mut set_b = HashSet::new();

    for n in a {
        *map_a.entry(n).or_insert(0) += 1;
    }

    for n in b {
        set_b.insert(n);
    }

    map_a.into_iter()
        .filter(|(k, _)| {
            set_b.get(k).is_some()
        })
        .map(|(_, v)| v)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 4, vec![2, 2, 3, 1], vec![2, 1, 4, 1], 3),
            TestCase(5, 3, vec![1, 1, 1, 1, 1], vec![1, 1, 1], 5),
            TestCase(10, 11, vec![7, 447, 71, 130, 24, 1, 2, 221, 71, 1334], vec![14, 93, 2000, 204, 447, 221, 7, 101, 7, 1, 30], 4),
        ];

        for TestCase(n, m, a, b, expected) in tests {
            assert_eq!(run(n, m, a, b), expected);
        }
    }
}
