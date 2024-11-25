// https://atcoder.jp/contests/joi2025yo1c/tasks/joi2025_yo1c_a

fn run(a: usize, b: usize) -> usize {
    a * 1000 + b * 10000
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(7, 2, 27000),
            TestCase(11, 1, 21000),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
