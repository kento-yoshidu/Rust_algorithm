// https://atcoder.jp/contests/abc164/tasks/abc164_c

use std::collections::HashSet;

pub fn run(_n: usize, s: Vec<&str>) -> usize {
    let mut hash_set = HashSet::new();

    for c in s {
        hash_set.insert(c);
    }

    hash_set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(3, vec!["apple", "orange", "apple"]));
        assert_eq!(1, run(5, vec!["grape", "grape", "grape", "grape", "grape"]));
        assert_eq!(4, run(4, vec!["aaaa", "a", "aaa", "aa"]));
    }
}
