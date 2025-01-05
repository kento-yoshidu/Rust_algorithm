// https://atcoder.jp/contests/abc387/tasks/abc387_a

fn run(a: usize, b: usize) -> usize {
    (a + b).pow(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(20, 25, 2025),
            TestCase(30, 25, 3025),
            TestCase(45, 11, 3136),
            TestCase(2025, 1111, 9834496),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
