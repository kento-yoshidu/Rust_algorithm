// https://atcoder.jp/contests/abc061/tasks/abc061_b

use std::collections::HashMap;

pub fn run(_n: usize, _m: usize, a: Vec<(usize, usize)>) -> Vec<usize> {
    let mut hashmap = HashMap::new();

    for t in a {
        *hashmap.entry(t.0).or_insert(0) += 1;
        *hashmap.entry(t.1).or_insert(0) += 1;
    }

    let mut vec: Vec<(&usize, &usize)> = hashmap.iter().collect();

    vec.sort_by(|a, b| (a.0).cmp(b.0));

    vec.iter()
        .map(|t| *t.1)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![2, 2, 1, 1], run(4, 3, vec![(1, 2), (2, 3), (1, 4)]));
        assert_eq!(vec![5, 5], run(2, 5, vec![(1, 2), (2, 1), (1, 2), (2, 1), (1, 2)]));
        assert_eq!(vec![3, 3, 2, 2, 2, 1, 1, 2], run(8, 8, vec![(1, 2), (3, 4), (1, 5), (2, 8), (3, 7), (5, 2), (4, 1), (6, 8)]));
    }
}
