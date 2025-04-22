// https://atcoder.jp/contests/abc055/tasks/abc055_b

fn run(n: usize) -> usize {
    (1..=n).fold(1, |state, i| { state * i % 1000000007 })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 6),
            TestCase(10, 3628800),
            TestCase(100000, 457992974),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
