// https://atcoder.jp/contests/abc055/tasks/arc069_a

pub fn run(n: usize, m: usize) -> usize {
    n + (m - n*2) / 4
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 6, 2),
            TestCase(12345, 678901, 175897),
        ];

        for TestCase(n, m, expected) in tests {
            assert_eq!(expected, run(n, m));
        }
    }
}
