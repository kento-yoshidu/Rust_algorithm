// https://atcoder.jp/contests/abc455/tasks/abc455_c

use std::collections::HashMap;

fn run(_n: usize, k: usize, a: Vec<usize>) -> usize {
    let mut map = HashMap::new();

    for a in a {
        *map.entry(a).or_insert(0) += 1;
    }

    let mut arr: Vec<usize> = map.into_iter()
        .map(|(x, count)| x * count)
        .collect();

    arr.sort_by(|a, b| b.cmp(a));

    let mut ans = arr.iter().sum();

    for i in 0..k.min(arr.len()) {
        ans -= arr[i];
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn abc455_c() {
        let tests = [
            TestCase(6,2, vec![7, 2, 7, 2, 2, 9], 6),
            TestCase(8, 6, vec![1, 2, 3, 4, 1, 2, 3, 4], 0),
            TestCase(10, 2, vec![3, 3, 4, 1, 1, 3, 3, 1, 5, 1], 8),
        ];

        for TestCase(n, k, a, expected) in tests {
            assert_eq!(run(n, k, a), expected);
        }
    }
}
