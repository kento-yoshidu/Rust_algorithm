// https://yukicoder.me/problems/no/830

fn run(n: usize) -> usize {
    n * 10
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn yuki_830() {
        let tests = [
            TestCase(9, 90),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
