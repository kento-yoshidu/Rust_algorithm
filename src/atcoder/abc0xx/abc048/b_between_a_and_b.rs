// https://atcoder.jp/contests/abc048/tasks/abc048_b

fn run(a: usize, b: usize, x: usize) -> usize {
    let i = (a + x - 1) / x;
    let j = b / x + 1;

    j - i
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 8, 2, 3),
            TestCase(0, 5, 1, 6),
            TestCase(9, 9, 2, 0),
            TestCase(1, 1000000000000000000, 3, 333333333333333333),
        ];

        for TestCase(a, b, x, expected) in tests {
            assert_eq!(run(a, b, x), expected);
        }
    }
}
