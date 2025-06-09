// https://atcoder.jp/contests/abc393/tasks/abc393_c

use std::collections::HashSet;

fn run(_n: usize, _m: usize, uv: Vec<(usize, usize)>) -> usize {
    let mut hash_set = HashSet::new();

    let mut ans = 0;

    for (u, v) in uv {
        if u == v {
            ans += 1;
            continue;
        }

        if hash_set.get(&(u, v)).is_some() || hash_set.get(&(v, u)).is_some() {
            ans += 1;
        } else {
            hash_set.insert((u, v));
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 5, vec![(1, 2), (2, 3), (3, 2), (3, 1), (1, 1)], 2),
            TestCase(6, 10, vec![(6, 2), (4, 1), (5, 1), (6, 6), (5, 3), (5, 1), (1, 4), (6, 4), (4, 2), (5, 6)], 3),
        ];

        for TestCase(n, m, uv, expected) in tests {
            assert_eq!(run(n, m, uv), expected);
        }
    }
}
