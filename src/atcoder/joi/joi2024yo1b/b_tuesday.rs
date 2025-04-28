// https://atcoder.jp/contests/joi2024yo1b/tasks/joi2024_yo1b_b

fn run(x: usize) -> usize {
    if x % 7 == 2 {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 1),
            TestCase(10, 0),
            TestCase(100, 1),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
