// https://atcoder.jp/contests/abc154/tasks/abc154_c

pub fn run(n: usize, a: Vec<usize>) -> &'static str {
    let mut vec = a.clone();
    vec.sort();
    vec.dedup();

    if vec.len() == n {
        "YES"
    } else {
        "NO"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, vec![2, 6, 1, 4, 5], "YES"),
            TestCase(6, vec![4, 1, 3, 1, 6, 2], "NO"),
            TestCase(2, vec![10000000, 10000000], "NO"),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
