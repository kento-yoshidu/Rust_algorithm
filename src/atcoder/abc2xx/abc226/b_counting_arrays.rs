// https://atcoder.jp/contests/abc226/tasks/abc226_b

use std::collections::HashSet;

fn run(_n: usize, a: Vec<Vec<usize>>) -> usize {
    let mut hash_set = HashSet::new();

    for vec in a.iter() {
        hash_set.insert(&vec[1..]);
    }

    hash_set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(4, vec![vec![2, 1, 2], vec![2, 1, 1], vec![2, 2, 1], vec![2, 1, 2]]));
        assert_eq!(4, run(5, vec![vec![1, 1], vec![1, 1], vec![1, 2], vec![2, 1, 1], vec![3, 1, 1, 1]]));
        assert_eq!(1, run(1, vec![vec![1, 1]]));
    }
}
