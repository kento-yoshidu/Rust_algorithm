// https://yukicoder.me/problems/no/279

use std::cmp::min;
use std::collections::HashMap;

fn run(s: &str) -> usize {
    let mut hash_map = HashMap::new();

    for c in s.chars() {
        *hash_map.entry(c).or_insert(0) += 1;
    }

    let t = hash_map.get(&'t').unwrap_or(&0);
    let r = hash_map.get(&'r').unwrap_or(&0);
    let e = hash_map.get(&'e').unwrap_or(&0);

    min(*t, min(*r, e/2))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn yuki_279() {
        let tests = [
            TestCase("takahashikunlovesyukicoder", 1),
            TestCase("treapisnotki", 0),
            TestCase("eerteerteerteert", 4),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
