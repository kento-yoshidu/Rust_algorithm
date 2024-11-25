// https://atcoder.jp/contests/joi2025yo1c/tasks/joi2025_yo1c_c

fn run(n: usize, a: usize, b: usize) -> usize {
    (1..=n)
        .filter(|num| (num % a == 0 && num % b != 0) || (num % a != 0 && num % b == 0))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, 2, 3, 3),
            TestCase(1, 5, 3, 0),
            TestCase(100, 1, 2, 50),
        ];

        for TestCase(n, a, b, expected) in tests {
            assert_eq!(run(n, a, b), expected);
        }
    }
}
