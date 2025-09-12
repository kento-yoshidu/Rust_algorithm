// https://atcoder.jp/contests/abc134/tasks/abc134_b

fn run(n: u32, d: u32) -> usize {
    (n as f64 / (d * 2 + 1) as f64).ceil() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(u32, u32, usize);

    #[test]
    fn abc134_b() {
        let tests = [
            TestCase(6, 2, 2),
            TestCase(14, 3, 2),
            TestCase(20, 4, 3),
        ];

        for TestCase(n, d, expected) in tests {
            assert_eq!(run(n, d), expected);
        }
    }
}
