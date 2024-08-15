// https://atcoder.jp/contests/arc033/tasks/arc033_1

fn run(n: usize) -> usize {
    n * (n+1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 1),
            TestCase(2, 3),
            TestCase(3, 6),
            TestCase(4, 10),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
