// https://atcoder.jp/contests/abc269/tasks/abc269_a

pub fn run(a: isize, b: isize, c: isize, d: isize) -> (isize, &'static str) {
    ((a + b) * (c - d), "Takahashi")
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize, (isize, &'static str));

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 2, 5, 3, (6, "Takahashi")),
            TestCase(10, -20, 30, -40, (-700, "Takahashi")),
            TestCase(100, 100, 100, -100, (40000, "Takahashi")),
        ];

        for TestCase(a, b, c, d, expected) in tests {
            assert_eq!(run(a, b, c, d), expected);
        }
    }
}
