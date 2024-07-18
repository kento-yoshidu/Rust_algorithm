// https://atcoder.jp/contests/arc026/tasks/arc026_1

fn run(n: usize, a: usize, b: usize) -> usize {
    if n < 5 {
        n * b
    } else {
        (5 * b) + ((n - 5) * a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(10, 5, 1, 30),
            TestCase(4, 60, 7, 28),
        ];

        for TestCase(n, a, b, expected) in tests {
            assert_eq!(run(n, a, b), expected);
        }
    }
}
