// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_bi

use itertools::Itertools;
use std::collections::HashMap;

pub fn run(_n: usize, _m: usize, ab: Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut hash_map = HashMap::new();

    for (a, b) in ab.iter() {
        hash_map.entry(*a).or_insert_with(|| Vec::new()).push(*b);
        hash_map.entry(*b).or_insert_with(|| Vec::new()).push(*a);
    }

    hash_map.into_iter()
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .map(|(_, mut vec)| {
            vec.sort();
            vec
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, Vec<Vec<usize>>);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 4, vec![(1, 2), (2, 3), (3, 4), (3, 5)], vec![vec![2], vec![1, 3], vec![2, 4, 5], vec![3], vec![3]]),
            TestCase(15, 30,   vec![(6, 9),
                                    (9, 10),
                                    (2, 9),
                                    (9, 12),
                                    (2, 14),
                                    (1, 4),
                                    (4, 6),
                                    (1, 3),
                                    (4, 14),
                                    (1, 6),
                                    (9, 11),
                                    (2, 6),
                                    (3, 9),
                                    (5, 9),
                                    (4, 9),
                                    (11, 15),
                                    (1, 13),
                                    (4, 13),
                                    (8, 9),
                                    (9, 13),
                                    (5, 15),
                                    (3, 5),
                                    (8, 10),
                                    (2, 4),
                                    (9, 14),
                                    (1, 9),
                                    (2, 8),
                                    (6, 13),
                                    (7, 9),
                                    (9, 15)],
                            vec![
                                vec![3, 4, 6, 9, 13],
                                vec![4, 6, 8, 9, 14],
                                vec![1, 5, 9],
                                vec![1, 2, 6, 9, 13, 14],
                                vec![3, 9, 15],
                                vec![1, 2, 4, 9, 13],
                                vec![9],
                                vec![2, 9, 10],
                                vec![1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 13, 14, 15],
                                vec![8, 9],
                                vec![9, 15],
                                vec![9],
                                vec![1, 4, 6, 9],
                                vec![2, 4, 9],
                                vec![5, 9, 11]]),
        ];

        for TestCase(n, m, ab, expected) in tests {
            assert_eq!(run(n, m, ab), expected);
        }
    }
}
