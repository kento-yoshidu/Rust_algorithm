// https://atcoder.jp/contests/abc069/tasks/abc069_a

fn run(n: usize, m: usize) -> usize {
    (n - 1) * (m - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 4, 6),
            TestCase(2, 2, 1),
        ];

        for TestCase(n, m ,expected) in tests {
            assert_eq!(run(n, m), expected);
        }
    }
}
