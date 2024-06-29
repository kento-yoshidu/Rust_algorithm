// https://atcoder.jp/contests/arc027/tasks/arc027_1

fn run(h: usize, m: usize) -> usize {
    (17 - h) * 60 + 60 - m
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(16, 23, 97),
            TestCase(17, 59, 1),
            TestCase(13, 0, 300),
            TestCase(2, 7, 953),
        ];

        for TestCase(h, m, expected) in tests {
            assert_eq!(run(h, m), expected);
        }
    }
}
