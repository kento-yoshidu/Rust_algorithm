// https://atcoder.jp/contests/abc155/tasks/abc155_c

use std::collections::HashMap;

fn run(_n: usize, h: Vec<&str>) -> Vec<String> {
    let mut hash_map = HashMap::new();

    for s in h {
        *hash_map.entry(s).or_insert(0) += 1;
    }

    let max_value = hash_map.values().max().unwrap();

    let mut ans = hash_map.iter()
        .filter(|e| e.1 == max_value)
        .map(|e| e.0.to_string())
        .collect::<Vec<String>>();

    ans.sort();

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, Vec<&'static str>);

    #[test]
    fn abc155_c() {
        let tests = [
            TestCase(7, vec!["beat", "vet", "beet", "bed", "vet", "bet", "beet"], vec!["beet", "vet"]),
            TestCase(8, vec!["buffalo", "buffalo", "buffalo", "buffalo", "buffalo", "buffalo", "buffalo", "buffalo"], vec!["buffalo"]),
            TestCase(7, vec!["bass", "bass", "kick", "kick", "bass", "kick", "kick"], vec!["kick"]),
            TestCase(4, vec!["ushi", "tapu", "nichia", "kun"], vec!["kun", "nichia", "tapu", "ushi"]),
        ];

        for TestCase(n, h, expected) in tests {
            assert_eq!(run(n, h), expected);
        }
    }
}
