// https://atcoder.jp/contests/abc046/tasks/abc046_b

pub fn run(n: usize, k: usize) -> usize {
    k * (k - 1).pow(n as u32 - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 2, 2),
            TestCase(1, 10, 10),
            TestCase(10, 8, 322828856),
        ];

        for TestCase(n, k, expected) in tests {
            assert_eq!(expected, run(n, k));
        }
    }
}
