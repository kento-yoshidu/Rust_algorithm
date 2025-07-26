// https://atcoder.jp/contests/abc331/tasks/abc331_c

use std::collections::BTreeMap;

fn run(n: usize, a: Vec<usize>) -> Vec<usize> {
    let mut btree_map = BTreeMap::new();

    for (i, n) in a.into_iter().enumerate() {
        btree_map.entry(n).or_insert_with(|| Vec::new()).push(i);
    }

    let mut ans = vec![0; n];

    let mut sum = 0;

    for (num, vec) in btree_map.into_iter().rev() {
        for i in vec.iter() {
            ans[*i] = sum;
        }

        sum += num * vec.len();
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>);

    #[test]
    fn abc331_c() {
        let tests = [
            TestCase(5, vec![1, 4, 1, 4, 2], vec![10, 0, 10, 0, 8]),
            TestCase(10, vec![31, 42, 59, 26, 53, 58, 97, 93, 23, 54], vec![456, 414, 190, 487, 361, 249, 0, 97, 513, 307]),
            TestCase(50, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
