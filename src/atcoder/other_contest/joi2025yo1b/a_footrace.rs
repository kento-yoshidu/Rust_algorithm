// https://atcoder.jp/contests/joi2025yo1b/tasks/joi2025_yo1b_a

fn run(t: usize, v: usize) -> usize {
    t * v
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 3, 15),
            TestCase(2, 4, 8),
        ];

        for TestCase(t, v, expected) in tests {
            assert_eq!(run(t, v), expected);
        }
    }
}
