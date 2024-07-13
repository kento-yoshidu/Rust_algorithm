// https://atcoder.jp/contests/keyence2019/tasks/keyence2019_a

fn run(n1: usize, n2: usize, n3: usize, n4: usize) -> &'static str {
    let mut vec = vec![n1, n2, n3, n4];

    vec.sort();

    if vec == [1, 4, 7, 9] {
        "YES"
    } else {
        "NO"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 7, 9, 4, "YES"),
            TestCase(1, 9, 7, 4, "YES"),
            TestCase(1, 2, 9, 1, "NO"),
            TestCase(4, 9, 0, 8, "NO"),
        ];

        for TestCase(n1, n2, n3, n4, expected) in tests {
            assert_eq!(run(n1, n2, n3, n4), expected);
        }
    }
}
