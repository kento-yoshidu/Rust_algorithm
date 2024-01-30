// https://atcoder.jp/contests/abc032/tasks/abc032_a

pub fn run(a: usize, b: usize, n: usize) -> usize {
    (n..).find(|i| {
        *i % a == 0 && *i % b == 0
    }).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 3, 8, 12),
            TestCase(2, 2, 2, 2),
            TestCase(12, 8, 25, 48),
        ];

        for TestCase(a, b, n, expected) in tests {
            assert_eq!(run(a, b, n), expected);
        }
    }
}
