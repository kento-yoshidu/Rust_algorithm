// https://yukicoder.me/problems/no/2706

fn run(a: usize, b: usize, x: usize) -> usize {
    (a + x - 1) / a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn yuki_2706() {
        let tests = [
            TestCase(12, 300, 1, 300),
            TestCase(10, 100, 11, 200),
        ];

        for TestCase(a, b, x, expected) in tests {
            assert_eq!(run(a, b, x), expected);
        }
    }
}
