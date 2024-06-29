// https://atcoder.jp/contests/nomura2020/tasks/nomura2020_a

fn run(h1: usize, m1: usize, h2: usize, m2: usize, k: usize) -> usize {
    ((h2 - h1) * 60 + (m2 - m1)) - k
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(10, 0, 15, 0, 30, 270),
            TestCase(10, 0, 12, 0, 120, 0),
        ];

        for TestCase(h1, m1, h2, m2, k, expected) in tests {
            assert_eq!(run(h1, m1, h2, m2, k), expected);
        }
    }
}
