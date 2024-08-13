// https://atcoder.jp/contests/abc204/tasks/abc204_a

fn run(a: usize, b: usize) -> usize {
    if a == b {
        a
    } else {
        3 - a - b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(0, 1, 2),
            TestCase(0, 0, 0),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
