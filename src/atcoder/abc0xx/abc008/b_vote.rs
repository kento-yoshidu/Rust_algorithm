// https://atcoder.jp/contests/abc008/tasks/abc008_2

use itertools::Itertools;

pub fn run(_n: usize, s: Vec<&str>) -> String {
    let hashmap = s.iter().counts();

    hashmap.iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("taro"), run(4, vec!["taro", "jiro", "taro", "saburo"]));
        assert_eq!(String::from("takahashikun"), run(1, vec!["takahashikun"]));
        assert_eq!(String::from("b"), run(9, vec!["a", "b", "c", "c", "b", "c", "b", "d", "e", "b"]));
    }
}
