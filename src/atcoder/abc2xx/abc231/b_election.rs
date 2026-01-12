// https://atcoder.jp/contests/abc231/tasks/abc231_b

use std::collections::HashMap;

fn run<'a>(_n: usize, s: Vec<&'a str>) -> &'a str {
    let mut map = HashMap::new();

    for name in s {
        *map.entry(name).or_insert(0) += 1;
    }

    map.into_iter()
        .max_by_key(|(_, v)| *v)
        .unwrap()
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, &'static str);

    #[test]
    fn abc231_b() {
        let tests = [
            TestCase(5, vec!["snuke", "snuke", "takahashi", "takahashi", "takahashi"], "takahashi"),
            TestCase(5, vec!["takahashi", "takahashi", "aoki", "takahashi", "snuke"], "takahashi"),
            TestCase(1, vec!["a"], "a"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
