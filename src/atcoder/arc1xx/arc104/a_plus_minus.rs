// https://atcoder.jp/contests/arc104/tasks/arc104_a

fn run(a: isize, b: isize) -> (isize, isize) {
    ((a + b) / 2, (a - b) / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, (isize, isize));

    #[test]
    fn test() {
        let tests = [
            TestCase(2, -2, (0, 2)),
            TestCase(3, 1, (2, 1)),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
