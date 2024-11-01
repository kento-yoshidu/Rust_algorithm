// https://atcoder.jp/contests/abc012/tasks/abc012_1

fn run(a: usize, b: usize) -> (usize, usize) {
    (b, a)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, (usize, usize));

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 2, (2, 1)),
            TestCase(5, 5, (5, 5)),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
