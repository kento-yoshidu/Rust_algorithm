// https://atcoder.jp/contests/abc182/tasks/abc182_a

fn run(a: usize, b: usize) -> usize {
    (a * 2 + 100) - b
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc182_a() {
        let tests = [
            TestCase(200, 300, 200),
            TestCase(10000, 0, 20100),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
