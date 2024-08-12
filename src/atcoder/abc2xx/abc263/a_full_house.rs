// https://atcoder.jp/contests/abc263/tasks/abc263_a

use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn run(a: usize, b: usize, c: usize, d: usize, e: usize) -> String {
    let vec = vec![a, b, c, d, e];
    let map = vec.iter().counts();
    let set: HashSet<usize> = map.into_values().collect();

    if set == HashSet::from([3, 2]) {
        String::from("Yes")
    } else {
        String::from("No")
    }
}

fn run2(a: usize, b: usize, c: usize, d: usize, e: usize) -> &'static str {
    let vec = vec![a, b, c, d, e];

    let mut hash_map = HashMap::new();

    for i in vec.into_iter() {
        *hash_map.entry(i).or_insert(0) += 1;
    }

    if hash_map.iter().any(|(_, v)| *v == 3) && hash_map.iter().any(|(_, v)| *v == 2) {
        "Yes"
    } else {
        "No"
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 2, 1, 2, 1, "Yes"),
            TestCase(12, 12, 11, 1, 2, "No"),
        ];

        for TestCase(a, b, c, d, e, expected) in tests {
            assert_eq!(run(a, b, c, d, e), expected);
            assert_eq!(run2(a, b, c, d, e), expected);
        }
    }
}
