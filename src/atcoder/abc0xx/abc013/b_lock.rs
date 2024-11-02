// https://atcoder.jp/contests/abc013/tasks/abc013_2

fn run(a: isize, b: isize) -> isize {
    ((b - a).abs()).min(10 - (b - a).abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 6, 2),
            TestCase(6, 4, 2),
            TestCase(8, 1, 3),
            TestCase(5, 6, 1),
            TestCase(0, 9, 1),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
