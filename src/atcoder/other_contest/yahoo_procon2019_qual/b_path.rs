// https://atcoder.jp/contests/yahoo-procon2019-qual/tasks/yahoo_procon2019_qual_b

use std::collections::HashMap;

pub fn run(ab: Vec<(usize, usize)>) -> &'static str {
    let mut hash_map = HashMap::new();

    for (a, b) in ab.iter() {
        *hash_map.entry(a).or_insert(0) += 1;
        *hash_map.entry(b).or_insert(0) += 1;
    }

    if hash_map.iter()
        .all(|(_, v)| *v != 3) {
            "YES"
        } else {
            "NO"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<(usize, usize)>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(vec![(4, 2), (1, 3), (2, 3)], "YES"),
            TestCase(vec![(3, 2), (2, 4), (1, 2)], "NO"),
            TestCase(vec![(2, 1), (3, 2), (4, 3)], "YES"),
        ];

        for TestCase(ab, expected) in tests {
            assert_eq!(run(ab), expected);
        }
    }
}
