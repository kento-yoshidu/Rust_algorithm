// https://atcoder.jp/contests/abc394/tasks/abc394_b

fn run(_n: usize, s: Vec<&str>) -> String {
    let mut s = s.clone();

    s.sort_by_key(|s| s.len());
    s.concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec!["tc", "oder", "a"], "atcoder"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
