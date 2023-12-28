// https://atcoder.jp/contests/arc001/tasks/arc001_1

use std::collections::HashMap;

pub fn run(_n: usize, s: &str) -> (usize, usize) {
    let mut hash_map = HashMap::new();

    for c in s.chars() {
        *hash_map.entry(c).or_insert(0) += 1;
    }

    let max = hash_map.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
    let mut min = hash_map.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap();

    if max == min {
        min = (&'0', &0);
    }

    (*max.1, *min.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!((4, 1), run(9, "131142143"));
        assert_eq!((5, 5), run(20, "12341234123412341234"));
        assert_eq!((4, 0), run(4, "1111"));
    }
}
