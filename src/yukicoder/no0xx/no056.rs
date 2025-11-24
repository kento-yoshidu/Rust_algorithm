// https://yukicoder.me/problems/no/56

fn run(d: usize, p: usize) -> usize {
	d + d * p / 100
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn yuki_056() {
        let tests = [
            TestCase(100, 8, 108),
            TestCase(10, 8, 10),
            TestCase(123, 10, 135),
            TestCase(25, 16, 29),
        ];

        for TestCase(d, p, expected) in tests {
            assert_eq!(run(d, p), expected);
        }
    }
}
