// https://atcoder.jp/contests/abc260/tasks/abc260_a

use std::collections::HashMap;

use itertools::Itertools;

pub fn run(s: &str) -> String {
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

fn run2(s: &str) -> String {
    let mut hash_map = HashMap::new();

    for c in s.chars() {
        *hash_map.entry(c).or_insert(0) += 1;
    }

    hash_map.into_iter()
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .find(|(_, cnt)| *cnt == 1)
        .map(|(c, _)| c.to_string())
        .unwrap_or(String::from("-1"))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("pop", "o"),
            TestCase("abc", "a"),
            TestCase("xxx", "-1"),
            TestCase("jfi", "f"),
            TestCase("mmd", "d"),
            TestCase("sus", "u"),
            TestCase("odd", "o"),
            TestCase("mad", "a"),
            TestCase("zza", "a"),
            TestCase("aza", "z"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
            assert_eq!(run2(s), expected);
        }
    }
}
