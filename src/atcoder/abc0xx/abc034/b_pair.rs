// https://atcoder.jp/contests/abc034/tasks/abc034_b

fn run(n: usize) -> usize {
    if n % 2 == 0 {
        n - 1
    } else {
        n + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(100, 99),
            TestCase(123456789, 123456790),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
