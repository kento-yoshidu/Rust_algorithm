// https://atcoder.jp/contests/past202206-open/tasks/past202206_b

use std::collections::HashMap;

pub fn run(s: String) -> String {
    let mut hashmap: HashMap<String, usize> = HashMap::new();

    for t in s.chars().collect::<Vec<_>>().windows(2) {
        *hashmap.entry(format!("{}{}", t[0], t[1])).or_insert(0) += 1;
    }

    // let mut tmp = hashmap.iter().map(|x| (!0 - x.1, x.0)).collect::<Vec<_>>();
    let mut tmp = hashmap.iter().collect::<Vec<_>>();

    tmp.sort();

    tmp[0].0.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("ab"), run(String::from("abcab")));
        assert_eq!(String::from("wv"), run(String::from("zyxwv")));
        assert_eq!(String::from("ii"), run(String::from("iiiiiiiiiiiiiiiiiiiiiiiiiiiiii")));
    }
}
