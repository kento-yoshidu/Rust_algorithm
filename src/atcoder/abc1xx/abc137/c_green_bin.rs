// https://atcoder.jp/contests/abc137/tasks/abc137_c

use std::collections::HashMap;
use itertools::Itertools;

fn run(_n: usize, s: Vec<&str>) -> usize {
    let mut hash_map = HashMap::new();

    for str in s.iter() {
        let s: String = str.chars().sorted().collect();

        *hash_map.entry(s).or_insert(0) += 1;
    }

    hash_map.into_iter()
        .map(|(_, v)| {
            if v > 1 {
                v * (v-1) / 2
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, usize);

    #[test]
    fn abc137_c() {
        let tests = [
            TestCase(3, vec!["acornistnt", "peanutbomb", "constraint"], 1),
            TestCase(2, vec!["oneplustwo", "ninemodsix"], 0),
            TestCase(5, vec!["abaaaaaaaa", "oneplustwo", "aaaaaaaaba", "twoplusone", "aaaabaaaaa"], 4),
            TestCase(2, vec!["aaaaaaaaaa", "aaaaaaaaab"], 0),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
