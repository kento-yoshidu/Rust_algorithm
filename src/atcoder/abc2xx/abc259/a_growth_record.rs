// https://atcoder.jp/contests/abc259/tasks/abc259_a

fn run(_n: usize, m: usize, x: usize, t: usize, d: usize) -> usize {
    if x <= m {
        return t
    }

    t - (x - m) * d
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(38, 20, 17, 168, 3, 168),
            TestCase(1, 0, 1, 3, 2, 1),
            TestCase(100, 10, 100, 180, 1, 90),
            TestCase(100, 99, 100, 200, 1, 199),
            TestCase(1, 0, 1, 2, 1, 1),
            TestCase(64, 17, 45, 177, 2, 121),
            TestCase(29, 5, 1, 180, 100, 180),
            TestCase(100, 0, 30, 200, 4, 80),
            TestCase(23, 15, 23, 199, 7, 143),
            TestCase(53, 27, 2, 108, 50, 108),
        ];

        for TestCase(n, m, x, t, d, expected) in tests {
            assert_eq!(run(n, m, x, t, d), expected);
        }
    }
}
