// https://atcoder.jp/contests/abc232/tasks/abc232_a

fn run(s: &str) -> u8 {
    let b = s.as_bytes();

    (b[0] - b'0') * (b[2] - b'0')
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, u8);

    #[test]
    fn abc232_a() {
        let tests = [
            TestCase("3x7", 21),
            TestCase("9x9", 81),
            TestCase("1x1", 1),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
