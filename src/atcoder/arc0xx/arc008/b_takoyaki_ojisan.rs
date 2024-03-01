// https://atcoder.jp/contests/arc007/tasks/arc007_2

use std::collections::HashMap;

fn run(_n: usize, _m: usize, name: &str, kit: &str) -> isize {
    let mut n_map = HashMap::new();

    for c in name.chars() {
        *n_map.entry(c).or_insert(0) += 1;
    }

    let mut k_map = HashMap::new();

    for c in kit.chars() {
        *k_map.entry(c).or_insert(0) += 1;
    }

    // nameの中に、kitに含まれない文字が一つでもあるなら不可能
    if n_map.iter()
        .any(|(k, _)| {
            !k_map.get(k).is_some()
        }) {
            return -1;
        }

    // nameとkitで各文字の個数の差を求める
    k_map.iter()
        .fold(1, |mut state, (k, v)| {
            if let Some(n) = n_map.get(k) {
                state += n - v
            }

            state
        }) as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str, &'static str, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(7, 26, "NAOHIRO", "ABCDEFGHIJKLMNOPQRSTUVWXYZ", 2),
            TestCase(8, 8, "TAKOYAKI", "TAKOYAKI", 1),
            TestCase(8, 4, "CHOKUDAI", "MYON", -1),
            TestCase(6, 6, "MONAKA", "NAMAKO", 1),
        ];

        for TestCase(n, m, name, kit, expected) in tests {
            assert_eq!(run(n, m, name, kit), expected);
        }
    }
}
