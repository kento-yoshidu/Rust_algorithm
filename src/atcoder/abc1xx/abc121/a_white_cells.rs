// https://atcoder.jp/contests/abc121/tasks/abc121_a

fn run(a: isize, b: isize, c: isize, d: isize) -> isize {
    (a - c) * (b - d)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize, isize);

    #[test]
    fn abc121_a() {
        let tests = [
            TestCase(3, 2, 2, 1, 1),
            TestCase(5, 5, 2, 3, 6),
            TestCase(2, 4, 2, 4, 0),
        ];

        for TestCase(a, b, c, d, expected) in tests {
            assert_eq!(run(a, b, c, d), expected);
        }
    }
}
