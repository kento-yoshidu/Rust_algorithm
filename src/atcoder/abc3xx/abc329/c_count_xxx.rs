// https://atcoder.jp/contests/abc329/tasks/abc329_c

use std::collections::HashMap;

pub fn run(n: usize, s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();

    let mut hashmap = HashMap::new();
    let mut count = 1;

    hashmap.insert(chars[0], 1);

    for i in 1..n {
        if chars[i] == chars[i-1] {
            count += 1;

            if count > *hashmap.get(&chars[i]).unwrap() {
                *hashmap.get_mut(&chars[i]).unwrap() += 1;
            }
        } else {
            count = 1;

            hashmap.entry(chars[i]).or_insert(1);
        }
    }

    hashmap.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run(6, "aaabaa"));
        assert_eq!(1, run(1, "x"));
        assert_eq!(8, run(12, "ssskkyskkkky"));
    }
}
