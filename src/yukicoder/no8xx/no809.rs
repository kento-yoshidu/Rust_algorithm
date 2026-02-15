// https://yukicoder.me/problems/no/809

fn run(c: usize) -> (usize, usize) {
    (1, c)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, (usize, usize));

    #[test]
    fn yuki_809() {
        let tests = [
            TestCase(6, (1, 6)),
        ];

        for TestCase(c, expected) in tests {
            assert_eq!(run(c), expected);
        }
    }
}
