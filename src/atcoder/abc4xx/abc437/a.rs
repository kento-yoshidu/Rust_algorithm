// https://atcoder.jp/contests/abc437/tasks/abc437_a

fn run(a: usize, b: usize) -> usize {
    a * 12 + b
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc437_a() {
        let tests = [
            TestCase(6, 7, 79),
            TestCase(4, 11, 59),
            TestCase(8, 0, 96),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
