// https://yukicoder.me/problems/no/192

fn run(n: usize) -> usize {
    n / 2 * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn yuki_192() {
        let tests = [
            TestCase(101, 100),
            TestCase(1000, 1000),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
