// https://atcoder.jp/contests/abc161/tasks/abc161_c

fn run(n: usize, k: usize) -> usize {
    let t = n % k;

    t.min(k - t)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc161_c() {
        let tests = [
            TestCase(7, 4, 1),
            TestCase(2, 6, 2),
            TestCase(1000000000000000000, 1, 0),
        ];

        for TestCase(n, k, expected) in tests {
            assert_eq!(run(n, k), expected);
        }
    }
}
