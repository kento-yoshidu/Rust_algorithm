// https://yukicoder.me/problems/no/46

fn run(a: usize, b: usize) -> usize {
    (b + a - 1) / a
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn yuki_046() {
        let tests = [
            TestCase(2, 5, 3),
            TestCase(10, 100, 10),
            TestCase(123456789, 987654321, 9),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
