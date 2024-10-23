// https://atcoder.jp/contests/joi2025yo1b/tasks/joi2025_yo1b_b

fn run(p: usize, q: usize, a: usize, b: usize) -> usize {
    if p > q {
        a * q
    } else {
        a * p + (q - p) * b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 5, 2, 1, 8),
            TestCase(3, 2, 2, 1, 4),
            TestCase(3, 3, 5, 5, 15),
        ];

        for TestCase(p, q, a, b, expected) in tests {
            assert_eq!(run(p, q, a, b), expected);
        }
    }
}
