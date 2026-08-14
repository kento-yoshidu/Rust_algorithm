// https://atcoder.jp/contests/abc036/tasks/abc036_a

fn run(a: usize, b: usize) -> usize {
    (b + a - 1) / a
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc036_a() {
        let tests = [
            TestCase(12, 36, 3),
            TestCase(12, 100, 9),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
