// https://atcoder.jp/contests/abc158/tasks/abc158_b

fn run(n: usize, a: usize, b: usize) -> usize {
    (n / (b + a)) * a + a.min(n % (b + a))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn abc158_b() {
        let tests = [
            TestCase(8, 3, 4, 4),
            TestCase(8, 0, 4, 0),
            TestCase(6, 2, 4, 2),
        ];

        for TestCase(n, a, b, expected) in tests {
            assert_eq!(run(n, a, b), expected);
        }
    }
}
