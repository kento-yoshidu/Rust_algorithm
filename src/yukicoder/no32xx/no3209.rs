// https://yukicoder.me/problems/no/3209

fn run(a: usize, b: usize) -> (usize, usize) {
    (10 - a, b - (10 - a))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, (usize, usize));

    #[test]
    fn yuki_3209() {
        let tests = [
            TestCase(5, 9, (5, 4)),
            TestCase(9, 9, (1, 8)),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
