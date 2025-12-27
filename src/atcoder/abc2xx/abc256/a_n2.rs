// https://atcoder.jp/contests/abc256/tasks/abc256_a

fn run(n: u32) -> u32 {
    2_u32.pow(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(u32, u32);

    #[test]
    fn abc256_a() {
        let tests = [
            TestCase(3, 8),
            TestCase(30, 1073741824),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
