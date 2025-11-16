// https://atcoder.jp/contests/abc213/tasks/abc213_a

fn run(a: usize, b: usize) -> usize {
    a ^ b
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc213_a() {
        let tests = [
            TestCase(3, 6, 5),
            TestCase(10, 12, 6),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
