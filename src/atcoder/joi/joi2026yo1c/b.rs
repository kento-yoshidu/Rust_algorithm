// https://atcoder.jp/contests/joi2026yo1c/tasks/joi2026_yo1c_b

fn run(x: usize, y: usize, n: usize) -> usize {
    if n % 2 == 0 {
        y * n
    } else {
        y * (n - 1) + x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn joi2026yo1c_b() {
        let tests = [
            TestCase(4, 3, 5, 16),
            TestCase(3, 1, 6, 6),
            TestCase(10, 2, 1, 10),
        ];

        for TestCase(x, y, n, expected) in tests {
            assert_eq!(run(x, y, n), expected);
        }
    }
}
