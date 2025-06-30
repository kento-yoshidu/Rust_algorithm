// https://atcoder.jp/contests/abc311/tasks/abc311_c

use std::collections::{HashMap, HashSet};

fn cycle(mut v: usize, hash_map: &HashMap<usize, usize>) -> Vec<usize> {
    for _ in 0..hash_map.len() {
        v = hash_map[&v];
    }

    let start = v;

    let mut res = vec![v];

    while let Some(&next) = hash_map.get(&v) {
        v = next;

        if v == start {
            break;
        }

        res.push(v);
    }

    res
}

fn run(_n: usize, a: Vec<usize>) -> (usize, Vec<usize>) {
    let mut hash_map = HashMap::new();

    for (i, &x) in a.iter().enumerate() {
        hash_map.insert(i + 1, x);
    }

    let res = cycle(1, &hash_map);

    (res.len(), res)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, (usize, Vec<usize>));

    #[test]
    fn test() {
        let tests = [
            TestCase(7, vec![6, 7, 2, 1, 3, 4, 5], (3, vec![6, 4, 1])),
            TestCase(8, vec![3, 7, 4, 7, 3, 3, 8, 2], (3, vec![2, 7, 8])),
            TestCase(2, vec![2, 1], (2, vec![1, 2])),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
