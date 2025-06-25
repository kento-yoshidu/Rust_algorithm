// https://atcoder.jp/contests/abc116/tasks/abc116_a

fn run(a: isize, b: isize, _c: isize) -> isize {
    a * b / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize);

    #[test]
    fn abc116_a() {
        let tests = [
            TestCase(3, 4, 5, 6),
            TestCase(5, 12, 13, 30),
            TestCase(45, 28, 53, 630),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
