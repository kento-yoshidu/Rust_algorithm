// https://atcoder.jp/contests/abc176/tasks/abc176_a

fn run(n: usize, x: usize, t: usize) -> usize {
    if n % x == 0 {
        n / x * t
    } else {
        (n / x) * t + t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn abc176_a() {
        let tests = [
            TestCase(10, 10, 5, 5),
            TestCase(20, 10, 5, 10),
            TestCase(20, 12, 6, 12),
            TestCase(1000, 1, 1000, 1000000),
        ];

        for TestCase(n, x, t, expected) in tests {
            assert_eq!(run(n, x, t), expected);
        }
    }
}
