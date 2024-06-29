// https://atcoder.jp/contests/arc048/tasks/arc048_a

fn run(a: isize, b: isize) -> isize {
    if  a < 0 && b < 0 ||
        a > 0 && b > 0 {
            (b - a).abs()
    } else {
        (b - a).abs() - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 6, 3),
            TestCase(-1, 1, 1),
            TestCase(-7, -2, 5),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
