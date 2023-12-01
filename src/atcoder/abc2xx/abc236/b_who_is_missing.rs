// https://atcoder.jp/contests/abc236/tasks/abc236_b

use std::collections::HashMap;

pub fn run(_n: usize, a: Vec<usize>) -> usize {
    let mut hash_map = HashMap::new();

    for i in a {
        *hash_map.entry(i).or_insert(0) += 1;
    }

    *hash_map.iter()
        .find(|(_, count)| {
            **count == 3
        })
        .unwrap()
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(3, vec![1, 3, 2, 3, 3, 2, 2, 1, 1, 1, 2]));
        assert_eq!(1, run(1, vec![1, 1, 1]));
        assert_eq!(2, run(4, vec![3, 2, 1, 1, 2, 4, 4, 4, 4, 3, 1, 3, 2, 1, 3]));
    }
}
