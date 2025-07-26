// https://atcoder.jp/contests/abc407/tasks/abc407_a

fn run(a: usize, b: usize) -> usize {
    (a as f64 / b as f64 + 0.5).floor() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc407_a() {
        let tests = [
            TestCase(4, 7, 1),
            TestCase(407, 29, 14),
            TestCase(22, 11, 2),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
