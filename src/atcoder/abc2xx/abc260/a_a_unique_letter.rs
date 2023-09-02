// https://atcoder.jp/contests/abc260/tasks/abc260_a

use std::collections::HashMap;

pub fn run(s: String) -> String {
    let mut map: HashMap<char, usize> = HashMap::new();

    for c in s.chars() {
        let counter = map.entry(c).or_insert(0);
        *counter += 1;
    }

    let mut vec: Vec<(&char, &usize)> = map.iter().collect();

    vec.sort_by(|a, b| a.0.cmp(b.0));

    for v in vec {
        if *v.1 == 1 {
            return v.0.to_string()
        }
    }

    String::from("-1")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("o"), run(String::from("pop")));
        assert_eq!(String::from("a"), run(String::from("abc")));
        assert_eq!(String::from("-1"), run(String::from("xxx")));
        assert_eq!(String::from("f"), run(String::from("jfi")));
        assert_eq!(String::from("d"), run(String::from("mmd")));
        assert_eq!(String::from("u"), run(String::from("sus")));
        assert_eq!(String::from("o"), run(String::from("odd")));
        assert_eq!(String::from("a"), run(String::from("mad")));
        assert_eq!(String::from("a"), run(String::from("zza")));
        assert_eq!(String::from("z"), run(String::from("aza")));
    }
}
