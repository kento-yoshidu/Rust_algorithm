// https://yukicoder.me/problems/no/835

fn run(n: usize) -> usize {
    (n as f64 * 1.5).floor() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn yuki_835() {
        let tests = [
            TestCase(1, 1),
            TestCase(2, 3),
            TestCase(3, 4),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
