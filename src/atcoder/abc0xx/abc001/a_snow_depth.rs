// https://atcoder.jp/contests/abc001/tasks/abc001_1

fn run(a: isize, b: isize) -> isize {
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(15, 10, 5),
            TestCase(0, 0, 0),
            TestCase(5, 20, -15),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
