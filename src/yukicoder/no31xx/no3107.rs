// https://yukicoder.me/problems/no/3107

fn run(n: u32) -> u32 {
    n.pow(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(u32, u32);

    #[test]
    fn yuki_3107() {
        let tests = [
            TestCase(45, 2025),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
