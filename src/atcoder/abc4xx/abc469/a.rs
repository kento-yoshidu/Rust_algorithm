// https://atcoder.jp/contests/abc469/tasks/abc469_a

fn run(n: usize, k: usize) -> usize {
    n - k + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc469_a() {
        let tests = [
            TestCase(5, 2, 4),
            TestCase(1, 1, 1),
            TestCase(99, 50, 50),
        ];

        for TestCase(n, k, expected) in tests {
            assert_eq!(run(n, k), expected);
        }
    }
}
