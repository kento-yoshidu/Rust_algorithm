// https://atcoder.jp/contests/abc255/tasks/abc255_a

use itertools::Itertools;

pub fn run(s: String) -> usize {
    s.chars().permutations(3).unique().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(String::from("aba")));
        assert_eq!(1, run(String::from("ccc")));
        assert_eq!(6, run(String::from("xyz")));
    }
}