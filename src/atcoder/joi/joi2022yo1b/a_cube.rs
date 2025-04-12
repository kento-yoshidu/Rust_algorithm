// https://atcoder.jp/contests/joi2022yo1b/tasks/joi2022_yo1b_a

fn run(x: usize) -> usize {
    x * x * x
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 64),
            TestCase(1, 1),
            TestCase(999, 997002999),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
