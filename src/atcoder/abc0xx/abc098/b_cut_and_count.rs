// https://atcoder.jp/contests/abc098/tasks/abc098_b

use std::collections::HashSet;

fn calc(s1: &str, s2: &str) -> usize {
    let set1: HashSet<_> = s1.chars().collect();
    let mut common_characters = HashSet::new();

    for c in s2.chars() {
        if set1.contains(&c) {
            common_characters.insert(c);
        }
    }

    common_characters.len()
}

pub fn run(n: usize, s: String) -> usize {
    *(1..n)
        .map(|i| {
            calc(&s[0..i], &s[i..])
        })
        .collect::<Vec<usize>>()
        .iter()
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(6, String::from("aabbca")));
        assert_eq!(1, run(10, String::from("aaaaaaaaaa")));
        assert_eq!(9, run(45, String::from("tgxgdqkyjzhyputjjtllptdfxocrylqfqjynmfbfucbir")));
    }
}
