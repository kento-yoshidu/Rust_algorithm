// https://atcoder.jp/contests/joi2021yo1b/tasks/joi2021_yo1b_a

fn run(a: usize, b: usize, c: usize) -> usize {
    if a <= c && c < b {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 5, 3, 1),
            TestCase(20, 22, 19, 0),
            TestCase(24, 30, 30, 0),
            TestCase(1, 100, 99, 1),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
