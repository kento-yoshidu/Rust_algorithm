// https://atcoder.jp/contests/abc210/tasks/abc210_c

use std::collections::HashMap;

fn run(n: usize, k: usize, c: Vec<usize>) -> usize {
    let mut hash_map = HashMap::new();

    for num in &c[0..k] {
        *hash_map.entry(num).or_insert(0) += 1;
    }

    let mut ans = hash_map.len();

    for i in k..n {
        *hash_map.entry(&c[i]).or_insert(0) += 1;

        if let Some(count) = hash_map.get_mut(&c[i-k]) {
            *count -= 1;

            if *count == 0 {
                hash_map.remove(&c[i-k]);
            }

            ans = ans.max(hash_map.len());
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
            TestCase(7, 3, vec![1, 2, 1, 2, 3, 3, 1], 3),
            TestCase(5, 5, vec![4, 4, 4, 4, 4], 1),
            TestCase(10, 6, vec![304621362, 506696497, 304621362, 506696497, 834022578, 304621362, 414720753, 304621362, 304621362, 414720753], 4),
        ];

        for TestCase(n, k, c, expected) in tests {
            assert_eq!(run(n, k, c), expected);
        }
    }
}
