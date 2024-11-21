// https://atcoder.jp/contests/abc251/tasks/abc251_c

use std::collections::HashSet;

fn run(_n: usize, st: Vec<(&str, usize)>) -> usize {
    let mut hash_set = HashSet::new();

    let mut pos = 0;
    let mut score = 0;

    for (i, (s, t)) in st.into_iter().enumerate() {
        if hash_set.contains(&s) {
            continue;
        }

        hash_set.insert(s);

        if t > score {
            pos = i+1;
            score = t;
        }
    }

    pos
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(&'static str, usize)>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![("aaa", 10), ("bbb", 20), ("aaa", 30)], 2),
        ];

        for TestCase(n, st, expected) in tests {
            assert_eq!(run(n, st), expected);
        }
    }
}
