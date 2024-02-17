// https://atcoder.jp/contests/abc081/tasks/arc086_a

use std::collections::HashMap;

pub fn run(n: usize, k: usize, a: Vec<usize>) -> usize {
    let mut hash_map = HashMap::new();

    for num in a {
        *hash_map.entry(num).or_insert(0) += 1;
    }

    let mut vec: Vec<(&usize, &usize)> = hash_map.iter().collect();
    vec.sort_by(|a, b| a.1.cmp(b.1));

    if k >= vec.len() {
        return 0;
    }

    let mut ans = 0;

    for i in 0..vec.len()-k {
        ans += vec[i].1;
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
            TestCase(5, 2, vec![1, 1, 2, 2, 5], 1),
            TestCase(4, 4, vec![1, 1, 2, 2], 0),
            TestCase(10, 3, vec![5, 1, 3, 2, 4, 1, 1, 2, 3, 4], 3),
            TestCase(100, 10, vec![52, 46, 61, 66, 97, 4, 64, 25, 52, 97, 5, 66, 97, 52, 66, 61, 63, 4, 64, 29, 18, 41, 64, 66, 65, 26, 29, 63, 24, 25, 41, 64, 64, 75, 67, 67, 97, 64, 29, 66, 63, 8, 97, 24, 24, 24, 63, 25, 5, 65, 5, 24, 67, 29, 25, 29, 25, 24, 23, 66, 29, 65, 29, 66, 61, 5, 64, 24, 75, 66, 24, 63, 63, 65, 66, 63, 5, 4, 52, 24, 52, 75, 66, 5, 24, 26, 41, 97, 5, 5, 97, 75, 29, 63, 4, 4, 5, 4, 52, 61], 24),
        ];

        for TestCase(n, k, a, expected) in tests {
            assert_eq!(run(n, k, a), expected);
        }
    }
}
