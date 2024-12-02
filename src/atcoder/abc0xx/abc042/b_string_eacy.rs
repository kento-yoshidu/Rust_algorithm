// https://atcoder.jp/contests/abc042/tasks/abc042_b

fn run(_n: usize, _l: usize, s: Vec<&str>) -> String {
    let mut vec = s.clone();
    vec.sort();

    vec.iter()
        .map(|str| str.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 3, vec!["dxx", "axx", "cxx"], "axxcxxdxx"),
        ];

        for TestCase(n, l, s, expected) in tests {
            assert_eq!(run(n, l, s), expected);
        }
    }
}
