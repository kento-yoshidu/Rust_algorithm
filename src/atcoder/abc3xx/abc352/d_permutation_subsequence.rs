// https://atcoder.jp/contests/abc352/tasks/abc352_d

use std::collections::BTreeSet;

pub fn run(n: usize, k: usize, p: Vec<usize>) -> usize {
    let mut vec = vec![0; n];

    for (i, num) in p.iter().enumerate() {
        vec[num-1] = i+1;
    }

    let mut btree_set = BTreeSet::new();

    let mut ans = std::usize::MAX;

    for num in vec.iter().take(k) {
        btree_set.insert(num);
    }

    for i in 0..n {
        if i >= k {
            btree_set.remove(&vec[i-k]);
        }

        btree_set.insert(&vec[i]);

        ans = ans.min(**btree_set.last().unwrap() - **btree_set.first().unwrap());
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
            TestCase(4, 2, vec![2, 3, 1, 4], 1),
            TestCase(4, 1, vec![2, 3, 1, 4], 0),
            TestCase(10, 5, vec![10, 1, 6, 8, 7, 2, 5, 9, 3, 4], 5),
        ];

        for TestCase(n, k, p, expected) in tests {
            assert_eq!(run(n, k, p), expected);
        }
    }
}
