// https://atcoder.jp/contests/abc159/tasks/abc159_a

fn run(n: isize, m: isize) -> isize {
    n * (n - 1) / 2 + m * (m - 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize);

    #[test]
    fn abc159_a() {
        let tests = [
            TestCase(2, 1, 1),
            TestCase(4, 3, 9),
            TestCase(1, 1, 0),
            TestCase(13, 3, 81),
            TestCase(0, 3, 3),
        ];

        for TestCase(n, m, expected) in tests {
            assert_eq!(run(n, m), expected);
        }
    }
}
