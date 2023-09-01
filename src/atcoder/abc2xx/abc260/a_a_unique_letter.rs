// https://atcoder.jp/contests/abc260/tasks/abc260_a

use std::collections::HashMap;

pub fn run(s: String) -> String {
    let mut map: HashMap<char, usize> = HashMap::new();

    for c in s.chars() {
        let counter = map.entry(c).or_insert(0);
        *counter += 1;
    }

    for m in map {
        if m.1 == 1 {
            return m.0.to_string()
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
        // assert_eq!(String::from("a"), run(String::from("abc")));
        assert_eq!(String::from("-1"), run(String::from("xxx")));
    }
}
