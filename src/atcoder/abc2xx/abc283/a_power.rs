// https://atcoder.jp/contests/abc283/tasks/abc283_a

fn run(a: usize, b: usize) -> usize {
    a.pow(b as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc283_a() {
        let tests = [
            TestCase(4, 3, 64),
            TestCase(5, 5, 3125),
            TestCase(8, 1, 8),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
