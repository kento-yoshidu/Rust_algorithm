// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_b

fn run(a1: usize, a2: usize, a3: usize) -> usize {
    a1 + a2 + a3
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn ma002() {
        let tests = [
            TestCase(10, 20, 50, 80),
            TestCase(1, 1, 1, 3),
            TestCase(100, 100, 100, 300),
        ];

        for TestCase(a1, a2, a3, expected) in tests {
            assert_eq!(run(a1, a2, a3), expected);
        }
    }
}
