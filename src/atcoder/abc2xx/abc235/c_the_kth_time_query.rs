// https://atcoder.jp/contests/abc235/tasks/abc235_c

use std::collections::HashMap;

pub fn run(_n: usize, _q: usize, a: Vec<usize>, xk: Vec<(usize, usize)>) -> Vec<isize> {
    let mut hash_map = HashMap::new();

    for (i, n) in a.iter().enumerate() {
        hash_map.entry(n).or_insert(Vec::new()).push(i+1);
    }

    xk.into_iter()
        .map(|(x, k)| {
            if let Some(arr) = hash_map.get(&x) {
                if let Some(i) = arr.get(k-1) {
                    *i as isize
                } else {
                    -1
                }
            } else {
                -1
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<(usize, usize)>, Vec<isize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, 8, vec![1, 1, 2, 3, 1, 2], vec![(1, 1), (1, 2), (1, 3), (1, 4), (2, 1), (2, 2), (2, 3), (4, 1)], vec![1, 2, 5, -1, 3, 6, -1, -1]),
            TestCase(3, 2, vec![0, 1000000000, 999999999], vec![(1000000000, 1), (123456789, 1)], vec![2, -1]),
        ];

        for TestCase(n, q, a, xk, expected) in tests {
            assert_eq!(run(n, q, a, xk), expected);
        }
    }
}
