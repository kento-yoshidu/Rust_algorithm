// https://atcoder.jp/contests/abc428/tasks/abc428_b

use std::collections::HashMap;
use itertools::Itertools;

fn run(n: usize, k: usize, s: &str) -> (usize, Vec<String>) {
    let chars: Vec<char> = s.chars().collect();

    let mut strs = HashMap::new();

    for i in 0..=n-k {
        let mut str = String::new();

        for j in 0..k {
            str.push(chars[i+j]);
        }

        *strs.entry(str).or_insert(0) += 1;
    }

    let max_count = strs.values().max().cloned().unwrap_or(0);

    let ans: Vec<String> = strs
        .into_iter()
        .filter(|(_, v)| *v == max_count)
        .map(|(k, _)| k)
        .sorted()
        .collect();

    (max_count, ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str, (usize, Vec<String>));

    #[test]
    fn abc428_b() {
        let tests = [
            TestCase(9, 3, "ovowowovo", (2, vec!["ovo".to_string(), "owo".to_string()])),
            TestCase(9, 1, "ovowowovo", (5, vec!["o".to_string()])),
            TestCase(35, 3, "thequickbrownfoxjumpsoverthelazydog", (2, vec!["the".to_string()])),
        ];

        for TestCase(n, k, s, expected) in tests {
            assert_eq!(run(n, k, s), expected);
        }
    }
}
