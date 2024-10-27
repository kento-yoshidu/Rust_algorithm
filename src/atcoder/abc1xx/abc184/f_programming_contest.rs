// https://atcoder.jp/contests/abc184/tasks/abc184_f

use itertools::Itertools;
use std::cmp::Ordering;

// upper_boundの拡張
// n以下の最大の数を返す
pub fn max_under_n<T: Ord>(vec: &[T], value: T) -> Option<usize> {
    vec.binary_search_by(|x| {
        if *x <= value {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    })
    .err()
    .map(|x| if x == 0 {
        None
    } else {
        Some(x - 1)
    })
    .flatten()
}

fn run(n: usize, t: usize, a: Vec<usize>) -> usize {
    let (l, r) = a.split_at(n/2);

    let mut p = Vec::new();
    let mut q = Vec::new();

    for i in 0..=l.len() {
        for combination in l.iter().combinations(i) {
            let sum: usize = combination.iter().map(|&&x| x).sum();
            p.push(sum);
        }
    }

    for i in 0..=r.len() {
        for combination in r.iter().combinations(i) {
            let sum: usize = combination.iter().map(|&&x| x).sum();
            q.push(sum)
        }
    }

    q.sort();

    let mut ans = 0;

    for left in p.iter() {
        if t < *left {
            continue;
        }

        if let Some(right_idx) = max_under_n(&q, t - left) {
            ans = ans.max(q[right_idx] + left)
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 17, vec![2, 3, 5, 7, 11], 17),
            TestCase(6, 100, vec![1, 2, 7, 5, 8, 10], 33),
            TestCase(6, 100, vec![101, 102, 103, 104, 105, 106], 0),
        ];

        for TestCase(n, t, a, expected) in tests {
            assert_eq!(run(n, t, a), expected);
        }
    }
}
