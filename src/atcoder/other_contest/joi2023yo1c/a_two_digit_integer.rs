// https://atcoder.jp/contests/joi2023yo1c/tasks/joi2023_yo1c_a

fn run(a: usize, b: usize) -> usize {
    a * 10 + b
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 2, 22),
            TestCase(1, 0, 10),
            TestCase(1, 9, 19),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
