// https://atcoder.jp/contests/abc231/tasks/abc231_b

use std::collections::HashMap;

pub fn run(_n: usize, s: Vec<&str>) -> String {
    let mut map = HashMap::new();

    for name in s {
        let count = map.entry(name).or_insert(0);
        *count += 1;
    }

    map.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("takahashi"), run(5, vec!["snuke", "snuke", "takahashi", "takahashi", "takahashi"]));
        assert_eq!(String::from("takahashi"), run(5, vec!["takahashi", "takahashi", "aoki", "takahashi", "snuke"]));
        assert_eq!(String::from("a"), run(1, vec!["a"]));
    }
}
