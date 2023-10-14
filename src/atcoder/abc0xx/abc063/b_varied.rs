// https://atcoder.jp/contests/abc063/tasks/abc063_b

use itertools::Itertools;

pub fn run(s: String) -> String {
    if s.chars().all_unique() {
        String::from("yes")
    } else {
        String::from("no")
    }
}

pub fn run2(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();

    chars.sort();
    chars.dedup();

    if s.len() == chars.len() {
        String::from("yes")
    } else {
        String::from("no")
    }
}

pub fn run3(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();

    chars.sort();

    if chars.windows(2).all(|v| {
        v[0] != v[1]
    }) {
        String::from("yes")
    } else {
        String::from("no")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("yes"), run(String::from("uncopyrightable")));
        assert_eq!(String::from("no"), run(String::from("different")));
        assert_eq!(String::from("yes"), run(String::from("no")));
    }

    #[test]
    fn test2() {
        assert_eq!(String::from("yes"), run2(String::from("uncopyrightable")));
        assert_eq!(String::from("no"), run2(String::from("different")));
        assert_eq!(String::from("yes"), run2(String::from("no")));
    }

    #[test]
    fn test3() {
        assert_eq!(String::from("yes"), run3(String::from("uncopyrightable")));
        assert_eq!(String::from("no"), run3(String::from("different")));
        assert_eq!(String::from("yes"), run3(String::from("no")));
    }
}
