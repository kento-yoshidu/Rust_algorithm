// https://atcoder.jp/contests/abc231/tasks/abc231_b

fn run(_n: usize, s: Vec<&str>) -> &str {
    use std::collections::HashMap;

    let mut hash = HashMap::new();

    for str in s.into_iter() {
        let counter = hash.entry(str).or_insert(0);
        *counter += 1;
    }

    hash.into_iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap().0
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, &'static str);

    #[test]
    fn abc312_b() {
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
