// https://atcoder.jp/contests/agc024/tasks/agc024_a

fn run(a: isize, b: isize, _c: isize, k: isize) -> isize {
    if k % 2 == 0 {
        a - b
    } else {
        b - a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 2, 3, 1, 1),
            TestCase(2, 3, 2, 0, -1),
            TestCase(2, 3, 2, 0, -1),
            TestCase(1000000000, 1000000000, 1000000000, 1000000000000000000, 0),
        ];

        for TestCase(a, b, c, k, expected) in tests {
            assert_eq!(run(a, b, c, k), expected);
        }
    }
}
