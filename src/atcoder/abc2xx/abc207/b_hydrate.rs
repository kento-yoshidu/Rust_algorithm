// https://atcoder.jp/contests/abc207/tasks/abc207_b

pub fn run(a: isize, b: isize, c: isize, d: isize) -> isize {
    if b >= c*d {
        -1
    } else {
        if a % ((b - c*d).abs()) == 0 {
            a / ((b - c*d).abs())
        } else {
            a / ((b - c*d).abs()) + 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(59575, 13178, 95470, 18045, 1),
            TestCase(31468, 50266, 8268, 88558, 1),
            TestCase(24334, 16907, 56812, 7, 1),
            TestCase(99725, 15162, 379, 40, -1),
            TestCase(99866, 19639, 6546, 3, -1),
            TestCase(99006, 83271, 41, 2031, -1),
            TestCase(99792, 61343, 5112, 12, 99792),
            TestCase(99525, 90728, 2, 45365, 49763),
            TestCase(100000, 100000, 100000, 100000, 1),
            TestCase(100000, 99999, 100, 1000, 100000),
            TestCase(100000, 1, 2, 1, 100000),
            TestCase(59575, 13178, 95470, 18045, 1),
            TestCase(5, 2, 3, 2, 2),
            TestCase(6, 9, 2, 3, -1),
        ];

        for TestCase(a, b, c, d, expected) in tests {
            assert_eq!(run(a, b, c, d), expected);
        }
    }
}
