// https://atcoder.jp/contests/abc071/tasks/arc081_a

use std::collections::HashMap;

fn run(_n: usize, a: Vec<usize>) -> usize {
    let mut hash_map = HashMap::new();

    for num in a.iter() {
        *hash_map.entry(num).or_insert(0) += 1;
    }

    let mut vec: Vec<(usize, usize)> = hash_map.iter()
        .map(|(k, v)| (**k, *v))
        .filter(|t| {
            t.1 > 1
        })
        .collect();

    if vec.len() == 0 {
        return 0;
    }

    vec.sort_by(|a, b| a.0.cmp(&b.0));
    vec.reverse();

    if vec[0].1 >= 4 {
        vec[0].0 * vec[0].0
    } else {
        vec[0].0 * vec[1].0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, vec![3, 1, 2, 4, 2, 1], 2),
            TestCase(4, vec![1, 2, 3, 4], 0),
            TestCase(10, vec![3, 3, 3, 3, 4, 4, 4, 5, 5, 5], 20),
            TestCase(4, vec![5, 5, 5, 5], 25),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
