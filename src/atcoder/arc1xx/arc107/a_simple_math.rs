// https://atcoder.jp/contests/arc107/tasks/arc107_a

pub fn run(a: usize, b: usize, c: usize) -> usize {
    let m = 998244353;

    let aa = a*(a+1)/2 % m;
    let bb = b*(b+1)/2 % m;
    let cc = c*(c+1)/2 % m;

    (aa * bb) % m * cc % m
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 2, 3, 18),
            TestCase(1000000000, 987654321, 123456789, 951633476),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
