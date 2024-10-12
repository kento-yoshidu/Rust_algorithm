// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_d

fn run(a1: usize, a2: usize, a3: usize) -> usize {
    a1 * a2 * a3
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 8, 8, 128),
            TestCase(7, 7, 25, 1225),
            TestCase(100, 100, 100, 1000000),
        ];

        for TestCase(a1, a2, a3, exepcted) in tests {
            assert_eq!(run(a1, a2, a3), exepcted);
        }
    }
}
