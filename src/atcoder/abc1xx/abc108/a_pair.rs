// https://atcoder.jp/contests/abc108/tasks/abc108_a

fn run(n: usize) -> usize {
    ((n + 1) / 2) * (n / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn abc108_a() {
        let tests = [
            TestCase(3, 2),
            TestCase(6, 9),
            TestCase(11, 30),
            TestCase(50, 625),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
