// https://atcoder.jp/contests/abc443/tasks/abc443_b

fn rec(i: usize, n: usize, sum: usize, k: usize) -> usize {
    if sum >= k {
        i
    } else {
        rec(i+1, n, n+i+1+sum, k)
    }
}

fn run(n: usize, k: usize) -> usize {
    rec(0, n, n, k)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc443_b() {
        let tests = [
            TestCase(4, 43, 6),
            TestCase(100000000, 100000000, 0),
            TestCase(1234, 12345678, 3886),
        ];

        for TestCase(n, k, expected) in tests {
            assert_eq!(run(n, k), expected);
        }
    }
}
