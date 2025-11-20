// https://atcoder.jp/contests/abc161/tasks/abc161_a

fn run(a: usize, b: usize, c: usize) -> (usize, usize, usize) {
    (c, a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, (usize, usize, usize));

    #[test]
    fn abc161_a() {
        let tests = [
            TestCase(1, 2, 3, (3, 1, 2)),
            TestCase(100, 100, 100, (100, 100, 100)),
            TestCase(41, 59, 31, (31, 41, 59)),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
