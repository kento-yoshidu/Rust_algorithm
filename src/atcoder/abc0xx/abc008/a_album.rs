// https://atcoder.jp/contests/abc008/tasks/abc008_1

fn run(s: usize, t: usize) -> usize {
    t - s + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 7, 4),
            TestCase(1, 1, 1),
            TestCase(999, 1000, 2),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
