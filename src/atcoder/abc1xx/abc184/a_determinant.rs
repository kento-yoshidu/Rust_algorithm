// https://atcoder.jp/contests/abc184/tasks/abc184_a

fn run(a: isize, b: isize, c: isize, d: isize) -> isize {
    a * d - b * c
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize, isize);

    #[test]
    fn abc184_a() {
        let tests = [
            TestCase(1, 2, 3, 4, -2),
            TestCase(0, -1, 1, 0, 1),
            TestCase(100, 100, 100, 100, 0),
        ];

        for TestCase(a, b, c, d, expected) in tests {
            assert_eq!(run(a, b, c, d), expected);
        }
    }
}
