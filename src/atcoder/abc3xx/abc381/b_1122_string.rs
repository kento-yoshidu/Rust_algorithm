//https://atcoder.jp/contests/abc381/tasks/abc381_b

use std::collections::HashMap;

fn run(s: &str) -> &'static str {
    for a in s.chars().collect::<Vec<char>>().windows(2).step_by(2) {
        if a[0] != a[1] {
            return "No";
        }
    }

    let mut hash_map = HashMap::new();

    for c in s.chars() {
        *hash_map.entry(c).or_insert(0) += 1;
    }

    if hash_map.values().all(|&v| v == 2) {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("aabbcc", "Yes"),
            TestCase("aab", "No"),
            TestCase("aaaa", "No"),
        ];

        for TestCase(s, expecetd) in tests {
            assert_eq!(run(s), expecetd);
        }
    }
}
