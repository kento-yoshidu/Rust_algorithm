// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_g

fn run(n: usize, x: usize, y: usize) -> usize {
    (1..=n)
        .filter(|num| {
            num % x == 0 || num % y == 0
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(15, 3, 5, 7),
            TestCase(1000000, 11, 13, 160839),
        ];

        for TestCase(n, x, y, expected) in tests {
            assert_eq!(run(n, x, y), expected);
        }
    }
}
