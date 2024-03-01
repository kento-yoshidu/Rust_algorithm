// https://atcoder.jp/contests/abc082/tasks/arc087_a

use std::collections::HashMap;

fn run(_n: usize, a: Vec<usize>) -> isize {
    let mut hash_map = HashMap::new();

    for num in a.iter() {
        *hash_map.entry(num).or_insert(0) += 1;
    }

    hash_map.into_iter()
        .fold(0, |state, (k, v)| {
            if v == *k {
                state
            } else if v > *k {
                state + (v as isize - *k as isize).abs()
            } else {
                state + v as isize
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![3, 3, 3, 3], 1),
            TestCase(5, vec![2, 4, 1, 4, 2], 2),
            TestCase(6, vec![1, 2, 2, 3, 3, 3], 0),
            TestCase(1, vec![1000000000], 1),
            TestCase(8, vec![2, 7, 1, 8, 2, 8, 1, 8], 5),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
