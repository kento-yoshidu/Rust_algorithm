// https://atcoder.jp/contests/abc061/tasks/abc061_b

use std::collections::HashMap;

fn run(_n: usize, _m: usize, a: &Vec<(usize, usize)>) -> Vec<usize> {
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

fn run2(n: usize, _m: usize, a: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut ans = vec![0; n];

    for (a, b) in a {
        ans[a-1] += 1;
        ans[b-1] += 1;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 2, vec![(1, 2)], vec![1, 1]),
            TestCase(4, 3, vec![(1, 2), (2, 3), (1, 4)], vec![2, 2, 1, 1]),
            TestCase(2, 5, vec![(1, 2), (2, 1), (1, 2), (2, 1), (1, 2)], vec![5, 5]),
            TestCase(8, 8, vec![(1, 2), (3, 4), (1, 5), (2, 8), (3, 7), (5, 2), (4, 1), (6, 8)], vec![3, 3, 2, 2, 2, 1, 1, 2]),
        ];

        for TestCase(n, m, a, expected) in tests {
            assert_eq!(run(n, m, &a), expected);
            assert_eq!(run2(n, m, &a), expected);
        }
    }
}
