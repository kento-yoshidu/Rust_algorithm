//  https://atcoder.jp/contests/abc109/tasks/abc109_b

use itertools::Itertools;

pub fn run(_n: usize, w: Vec<&str>) -> String {
    if !w.iter().all_unique() {
        return String::from("No")
    }

    if w.windows(2)
        .all(|t| {
            t[0].chars().last().unwrap() == t[1].chars().nth(0).unwrap()
        }) {
            String::from("Yes")
        } else {
            String::from("No")
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("No"), run(4, vec!["hoge", "english", "hoge", "enigma"]));
        assert_eq!(String::from("Yes"), run(9, vec!["basic", "c", "cpp", "php", "python", "nadesico", "ocaml", "lua", "assembly"]));
        assert_eq!(String::from("No"), run(8, vec!["a", "aa", "aaa", "aaaa", "aaaaa", "aaaaaa", "aaa", "aaaaaaa"]));
        assert_eq!(String::from("No"), run(3, vec!["abc", "arc", "agc"]));
    }
}
