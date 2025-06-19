// https://atcoder.jp/contests/abc397/tasks/abc397_c

use std::collections::HashSet;

fn run(n: usize, a: Vec<usize>) -> usize {
    let mut l = Vec::new();
    let mut l_set = HashSet::new();

    for i in 0..n-1 {
        l_set.insert(a[i]);
        l.push(l_set.len());
    }

    let mut r = Vec::new();
    let mut r_set = HashSet::new();

    for i in (1..n).rev() {
        r_set.insert(a[i]);
        r.push(r_set.len());
    }

    l.into_iter()
        .zip(r.into_iter().rev())
        .map(|(l, r)| l + r)
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc397_c() {
        let tests = [
            TestCase(5, vec![3, 1, 4, 1, 5], 5),
            TestCase(10, vec![2, 5, 6, 5, 2, 1, 7, 9, 7, 2], 8),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
