// https://atcoder.jp/contests/abc149/tasks/abc149_b

fn run(a: usize, b: usize, k: usize) -> (usize, usize) {
    if a >= k {
        (a - k, b)
    } else if  k >= a + b {
        (0, 0)
    } else {
        (0, b - (k - a))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, (usize, usize));

    #[test]
    fn abc149_b() {
        let tests = [
            TestCase(2, 3, 3, (0, 2)),
            TestCase(500000000000, 500000000000, 1000000000000, (0, 0)),
            TestCase(500000000000, 499999999999, 1000000000000, (0, 0)),
        ];

        for TestCase(a, b, k, expected) in tests {
            assert_eq!(run(a, b, k), expected);
        }
    }
}
