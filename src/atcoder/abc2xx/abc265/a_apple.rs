// https://atcoder.jp/contests/abc265/tasks/abc265_a

fn run(x: usize, y: usize, n: usize) -> usize {
    let tmp = (n / 3) * y;
    let tmp_a = tmp + (n % 3) * x;

    tmp_a.min(x * n)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn abc265_a() {
        let tests = [
            TestCase(10, 25, 10, 85),
            TestCase(10, 40, 10, 100),
            TestCase(100, 100, 2, 200),
            TestCase(100, 100, 100, 3400),
        ];

        for TestCase(x, y, n, expected) in tests {
            assert_eq!(run(x, y, n), expected);
        }
    }
}
