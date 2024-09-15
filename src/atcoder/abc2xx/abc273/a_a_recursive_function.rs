// https://atcoder.jp/contests/abc273/tasks/abc273_a

fn calc(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        n * calc(n - 1)
    }
}

fn run(n: usize) -> usize {
    calc(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 2),
            TestCase(3, 6),
            TestCase(0, 1),
            TestCase(10, 3628800),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
