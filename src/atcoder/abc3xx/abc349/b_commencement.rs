// https://atcoder.jp/contests/abc349/tasks/abc349_b

use std::collections::HashMap;

pub fn run(s: &str) -> &'static str {
    let chars: Vec<char> = s.chars().collect();
    let mut hash_map = HashMap::new();

    for c in chars {
        *hash_map.entry(c).or_insert(0) += 1;
    }

    let mut count = vec![0; 100];

    for (_, num) in hash_map {
        count[num-1] += 1;
    }

    if count.into_iter()
        .all(|cnt| {
            cnt == 0 || cnt == 2
        }) {
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
            TestCase("commencement", "Yes"),
            TestCase("banana", "No"),
            TestCase("ab", "Yes"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
