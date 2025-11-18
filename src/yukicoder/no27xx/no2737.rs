// https://yukicoder.me/problems/no/2737

use std::collections::HashSet;

fn run(n: usize, s: Vec<&str>) -> usize {
    let mut hash_set = HashSet::new();

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            hash_set.insert(format!("{}{}", s[i], s[j]));
        }
    }

    hash_set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, usize);

    #[test]
    fn yuki_2737() {
        let tests = [
            TestCase(2, vec!["pon", "juice"], 2),
            TestCase(2, vec!["pon", "ponpon"], 1),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
