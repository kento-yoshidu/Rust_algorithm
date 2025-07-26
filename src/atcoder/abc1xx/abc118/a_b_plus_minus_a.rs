// https://atcoder.jp/contests/abc118/tasks/abc118_a

fn run(a: usize, b: usize) -> usize {
    if b % a == 0 {
        return a + b
    }

    b - a
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc118_a() {
        let tests = [
            TestCase(4, 12, 16),
            TestCase(8, 20, 12),
            TestCase(1, 1, 2),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
