// https://atcoder.jp/contests/abc186/tasks/abc186_a

pub fn run(n: usize, w: usize) -> usize {
    n  / w
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(10, 3, 3),
            TestCase(1000, 1, 1000),
        ];

        for TestCase(n, w, expected) in tests {
            assert_eq!(run(n, w), expected);
        }
    }
}
