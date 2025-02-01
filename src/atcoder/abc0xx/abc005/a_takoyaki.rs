// https://atcoder.jp/contests/abc005/tasks/abc005_1

fn run(x: usize, y: usize) -> usize {
    y / x
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 8, 2),
            TestCase(4, 7, 1),
            TestCase(4, 3, 0),
        ];

        for TestCase(x, y, expected) in tests {
            assert_eq!(run(x, y), expected);
        }
    }
}
