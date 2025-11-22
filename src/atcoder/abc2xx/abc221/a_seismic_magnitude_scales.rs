// https://atcoder.jp/contests/abc221/tasks/abc221_a

fn run(a: u32, b: u32) -> u32 {
    32_u32.pow(a - b)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(u32, u32, u32);

    #[test]
    fn abc221_a() {
        let tests = [
            TestCase(6, 4, 1024),
            TestCase(1, 1, 1),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
