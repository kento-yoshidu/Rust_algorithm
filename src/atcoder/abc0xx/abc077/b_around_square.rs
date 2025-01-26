// https://atcoder.jp/contests/abc077/tasks/abc077_b

fn run(n: usize) -> usize {
    ((n as f64).sqrt() as u32).pow(2) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(10, 9),
            TestCase(81, 81),
            TestCase(271828182, 271821169),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
