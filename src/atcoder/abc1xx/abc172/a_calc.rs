// https://atcoder.jp/contests/abc172/tasks/abc172_a

fn run(a: usize) -> usize {
    a + a * a + a * a * a
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn abc172_a() {
        let tests = [
            TestCase(2, 14),
            TestCase(10, 1110),
        ];

        for TestCase(a, expected) in tests {
            assert_eq!(run(a), expected);
        }
    }
}
