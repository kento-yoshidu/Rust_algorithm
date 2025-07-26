// https://atcoder.jp/contests/abc398/tasks/abc398_c

use std::collections::HashMap;

fn run(_n: usize, a: Vec<usize>) -> isize {
    let mut hash_map = HashMap::new();

    for (i, &n) in a.iter().enumerate() {
        hash_map.entry(n)
            .and_modify(|e: &mut (usize, usize)| e.0 += 1)
            .or_insert((1, i + 1));
    }

    let Some((_, &(_, index))) = hash_map.iter()
        .filter(|(_, &(count, _))| count == 1)
        .max_by_key(|&(k, _)| *k)
    else {
        return -1;
    };

    index as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, isize);

    #[test]
    fn abc398_c() {
        let tests = [
            TestCase(9, vec![2, 9, 9, 7, 9, 2, 4, 5, 8], 9),
            TestCase(4, vec![1000000000, 1000000000, 998244353, 998244353], -1),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
