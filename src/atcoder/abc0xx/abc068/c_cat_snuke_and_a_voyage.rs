// https://atcoder.jp/contests/abc068/tasks/arc079_a

use std::collections::HashSet;

fn run(n: usize, _m: usize, ab: Vec<(usize, usize)>) -> &'static str {
    let mut vec = vec![HashSet::new(); n];

    for (a, b) in ab.iter() {
        vec[a-1].insert(b-1);
        vec[b-1].insert(a-1);
    }

    for i in vec[0].iter() {
        for num in vec[*i].iter() {
            if *num == n-1 {
                return "POSSIBLE"
            }
        }
    }

    "IMPOSSIBLE"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 2, vec![(1, 2), (2, 3)], "POSSIBLE"),
            TestCase(4, 3, vec![(1, 2), (2, 3), (3, 4)], "IMPOSSIBLE"),
            TestCase(100000, 1, vec![(1, 99999)], "IMPOSSIBLE"),
            TestCase(5, 5, vec![(1, 3), (4, 5), (2, 3), (2, 4), (1, 4)], "POSSIBLE"),
        ];

        for TestCase(n, m, ab, expected) in tests {
            assert_eq!(run(n, m, ab), expected);
        }
    }
}
