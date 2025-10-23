// https://atcoder.jp/contests/abc180/tasks/abc180_a

fn run(n: isize, a: isize, b: isize) -> isize {
    n - a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize);

    #[test]
    fn abc180_a() {
        let tests = [
            TestCase(100, 1, 2, 101),
            TestCase(100, 2, 1, 99),
            TestCase(100, 1, 1, 100),
        ];

        for TestCase(n, a, b, expected) in tests {
            assert_eq!(run(n, a, b), expected);
        }
    }
}
