// https://atcoder.jp/contests/abc166/tasks/abc166_b

use std::collections::HashSet;

pub fn run(n: usize, _k: usize, vec: Vec<(usize, Vec<usize>)>) -> usize {
    let mut hash_set = HashSet::new();

    for v in vec {
        for num in v.1 {
            hash_set.insert(num);
        }
    }

    (1..=n)
        .filter(|num| {
            !hash_set.contains(num)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(3, 2, vec![(2, vec![1, 3]), (1, vec![3])]));
        assert_eq!(2, run(3, 3, vec![(1, vec![3]), (1, vec![3]), (1, vec![3])]));
    }
}
