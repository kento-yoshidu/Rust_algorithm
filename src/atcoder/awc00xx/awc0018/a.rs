// https://atcoder.jp/contests/awc0018/tasks/awc0018_a

fn run(n: usize, m: usize) -> usize {
    2 * (n - 1) * m
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn awc0018_a() {
        let tests = [
            TestCase(3, 1, 4),
            TestCase(5, 4, 32),
            TestCase(1000000000, 1000000000, 1999999998000000000),
        ];

        for TestCase(n, m, expected) in tests {
            assert_eq!(run(n, m), expected);
        }
    }
}
