// https://atcoder.jp/contests/abc148/tasks/abc148_c

fn gcd(m: usize, n: usize) -> usize {
    if n == 0 {
        m
    } else {
        gcd(n, m % n)
    }
}

fn run(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc148_c() {
        let tests = [
            TestCase(2, 3, 6),
            TestCase(123, 456, 18696),
            TestCase(100000, 99999, 9999900000),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
