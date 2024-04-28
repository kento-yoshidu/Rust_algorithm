// https://atcoder.jp/contests/aising2019/tasks/aising2019_a

pub fn run(n: usize, h: usize, w: usize) -> usize {
    (n-h+1) * (n-w+1)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 2, 3, 2),
            TestCase(100, 1, 1, 10000),
            TestCase(5, 4, 2, 8),
        ];

        for TestCase(n, h, w, expected) in tests {
            assert_eq!(run(n, h, w), expected);
        }
    }
}
