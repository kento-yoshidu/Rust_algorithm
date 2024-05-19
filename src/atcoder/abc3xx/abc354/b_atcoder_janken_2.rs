// https://atcoder.jp/contests/abc354/tasks/abc354_b

pub fn run<'a>(n: usize, sc: Vec<(&'a str, usize)>) -> &'a str {
    let mut vec = sc.clone();

    let sum: usize = vec.iter().map(|(_, s)| s).sum();

    vec.sort();

    vec[sum % n].0
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(&'static str, usize)>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![("takahashi", 2), ("aoki", 6), ("snuke", 5)], "snuke"),
            TestCase(3, vec![("takahashi", 2813), ("takahashixx", 1086), ("takahashix", 4229)], "takahashix"),
        ];

        for TestCase(n, sc, expected) in tests {
            assert_eq!(run(n, sc), expected);
        }
    }
}
