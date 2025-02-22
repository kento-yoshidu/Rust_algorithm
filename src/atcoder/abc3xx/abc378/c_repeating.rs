// https://atcoder.jp/contests/abc378/tasks/abc378_c

use std::{collections::HashMap, hash::Hash};

fn run(_n: usize, a: Vec<isize>) -> Vec<isize> {
    let mut hash_map: HashMap<isize, isize> = HashMap::new();

    let mut ans = Vec::new();

    for (i, n) in a.iter().enumerate() {
        let value = *hash_map.get(&n).unwrap_or(&(-1));

        ans.push(value);

        hash_map.insert(*n, i as isize + 1);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, Vec<isize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, vec![1, 2, 1, 1, 3], vec![-1, -1, 1, 3, -1]),
            TestCase(4, vec![1, 1000000000, 1000000000, 1], vec![-1, -1, 2, 1]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
