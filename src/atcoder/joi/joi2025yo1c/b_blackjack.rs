// https://atcoder.jp/contests/joi2025yo1c/tasks/joi2025_yo1c_b

fn run(a: usize, b: usize, c: usize) -> usize {
    if a + b + c <= 21 {
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
            TestCase(5, 10, 6, 1),
            TestCase(7, 8, 13, 0),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
