// https://atcoder.jp/contests/abc008/tasks/abc008_2

use itertools::Itertools;

fn run<'a>(_n: usize, s: Vec<&'a str>) -> &'a str {
    let hashmap = s.iter().counts();

    hashmap.iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec!["taro", "jiro", "taro", "saburo"], "taro"),
            TestCase(1, vec!["takahashikun"], "takahashikun"),
            TestCase(9, vec!["a", "b", "c", "c", "b", "c", "b", "d", "e", "b"], "b"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
