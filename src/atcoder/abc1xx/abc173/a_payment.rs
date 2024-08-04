// https://atcoder.jp/contests/abc173/tasks/abc173_a

fn run(n: usize) -> usize {
    (n as f64 / 1000.0).ceil() as usize * 1000 - n
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(1900, 100),
            TestCase(3000, 0),
            TestCase(3525, 475),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
