// https://atcoder.jp/contests/abc022/tasks/abc022_a

fn run(a: usize) -> usize {
    (a / 2) * (a / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(10, 25),
            TestCase(60, 900),
        ];

        for TestCase(a, expected) in tests {
            assert_eq!(run(a), expected);
        }
    }
}
