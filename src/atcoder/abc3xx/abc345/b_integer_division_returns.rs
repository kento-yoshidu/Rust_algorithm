// https://atcoder.jp/contests/abc345/tasks/abc345_b

fn run(x: isize) -> isize {
    if x % 10 == 0 || x < 0 {
        x / 10
    } else {
        x / 10 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(27, 3),
            TestCase(-13, -1),
            TestCase(40, 4),
            TestCase(-20, -2),
            TestCase(123456789123456789, 12345678912345679),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
