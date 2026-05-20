// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/1/ITP1_1_B

fn run(n: usize) -> usize {
    n.pow(3)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn itp1_1_b() {
        let tests = [
            TestCase(2, 8),
            TestCase(3, 27),
            TestCase(64, 262144),
            TestCase(100, 1000000),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
