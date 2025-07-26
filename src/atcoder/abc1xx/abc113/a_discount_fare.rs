// https://atcoder.jp/contests/abc113/tasks/abc113_a

fn run(x: usize, y: usize) -> usize {
    x + y / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc113_a() {
        let tests = [
            TestCase(81, 58, 110),
            TestCase(4, 54, 31),
        ];

        for TestCase(x, y, expected) in tests {
            assert_eq!(run(x, y), expected);
        }
    }
}
