// https://atcoder.jp/contests/abc022/tasks/abc022_b

use std::collections::HashSet;

fn run(_n: usize, a: Vec<usize>) -> usize {
    let mut hash_set = HashSet::new();

    let mut ans = 0;

    for i in a {
        if hash_set.contains(&i) {
            ans += 1;
        } else {
            hash_set.insert(i);
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, vec![1, 2, 3, 2, 1], 2),
            TestCase(11, vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5], 4),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(expected, run(n, a));
        }
    }
}
