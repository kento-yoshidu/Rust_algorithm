// https://atcoder.jp/contests/abc318/tasks/abc318_a

fn run(n: isize, m: isize, p: isize) -> isize {
    if n - m < 0 {
        return 0;
    }

    (n - m) / p + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(13, 3, 5, 3),
            TestCase(5, 6, 6, 0),
            TestCase(200000, 314, 318, 628),
        ];

        for TestCase(n, m, p, expected) in tests {
            assert_eq!(run(n, m, p), expected);
        }
    }
}
