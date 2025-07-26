// https://atcoder.jp/contests/abc413/tasks/abc413_b

use std::collections::HashSet;

fn run(n: usize, s: Vec<&str>) -> usize {
    let mut hash_set = HashSet::new();

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            let str = format!("{}{}", &s[i], &s[j]);

            hash_set.insert(str);
        }
    }

    hash_set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, usize);

    #[test]
    fn abc413_b() {
        let tests = [
            TestCase(4, vec!["at", "atco", "coder", "der"], 11),
            TestCase(5, vec!["a", "aa", "aaa", "aaaa", "aaaaa"], 7),
            TestCase(10, vec!["armiearggc", "ukupaunpiy", "cogzmjmiob", "rtwbvmtruq", "qapfzsitbl", "vhkihnipny", "ybonzypnsn", "esxvgoudra", "usngxmaqpt", "yfseonwhgp"], 90),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
