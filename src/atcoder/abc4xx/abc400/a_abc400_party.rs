// https://atcoder.jp/contests/abc400/tasks/abc400_a

fn run(a: isize) -> isize {
    if 400 % a == 0 {
        400 / a
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(10, 40),
            TestCase(11, -1),
            TestCase(400, 1),
        ];

        for TestCase(a, expected) in tests {
            assert_eq!(run(a), expected);
        }
    }
}
