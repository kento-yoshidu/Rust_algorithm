// https://atcoder.jp/contests/abc155/tasks/abc155_c

use std::collections::HashMap;

pub fn run(_n: usize, h: Vec<&str>) -> Vec<String> {
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

    #[test]
    fn test() {
        assert_eq!(vec!["beet", "vet"], run(7, vec!["beat", "vet", "beet", "bed", "vet", "bet", "beet"]));
        assert_eq!(vec!["buffalo"], run(8, vec!["buffalo", "buffalo", "buffalo", "buffalo", "buffalo", "buffalo", "buffalo", "buffalo"]));
        assert_eq!(vec!["kick"], run(7, vec!["bass", "bass", "kick", "kick", "bass", "kick", "kick"]));
        assert_eq!(vec!["kun", "nichia", "tapu", "ushi"], run(4, vec!["ushi", "tapu", "nichia", "kun"]));
    }
}
