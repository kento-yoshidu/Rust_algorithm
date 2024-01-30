// https://atcoder.jp/contests/abc215/tasks/abc215_c

use itertools::Itertools;

pub fn run(s: &str, k: usize) -> String {
    let chars: Vec<char> = s.chars().collect();

    let mut vec: Vec<_> = Vec::new();

    for a in chars.iter().permutations(s.len()) {
        vec.push(a.iter().map(|c| c.to_string()).collect::<String>());
    }

    vec.sort();
    vec.dedup();

    vec[k-1].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("aba"), run("aab", 2));
        assert_eq!(String::from("baab"), run("baba", 4));
        assert_eq!(String::from("zyxwdcba"), run("ydxwacbz", 40320));
    }
}
