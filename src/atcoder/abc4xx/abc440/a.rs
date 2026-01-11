// https://atcoder.jp/contests/abc440/tasks/abc440_a

fn run(x: isize, y: u32) -> isize {
    x * 2_isize.pow(y)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, u32, isize);

    #[test]
    fn abc440_a() {
        let tests = [
            TestCase(110, 2, 440),
            TestCase(233, 3, 1864),
            TestCase(432, 1, 864),
        ];

        for TestCase(x, y, expected) in tests {
            assert_eq!(run(x, y), expected);
        }
    }
}
