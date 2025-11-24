// https://atcoder.jp/contests/abc169/tasks/abc169_a

fn run(a: usize, b: usize) -> usize {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc169_a() {
        let tests = [
            TestCase(2, 5, 10),
            TestCase(100, 100, 10000),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
