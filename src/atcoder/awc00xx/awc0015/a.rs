// https://atcoder.jp/contests/awc0015/tasks/awc0015_a

fn run(a: [isize; 7], b: [isize; 7]) -> isize {
    a.into_iter()
        .zip(b)
        .map(|(a, b)| a * b)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase([isize; 7], [isize; 7], isize);

    #[test]
    fn awc0015_a() {
        let tests = [
            TestCase([1, 2, 3, 4, 5, 6, 7], [1, 1, 1, 1, 1, 1, 1], 28),
            TestCase([3, -1, 4, 1, -5, 9, 2], [2, 7, -1, 8, 2, -3, 6], -22),
            TestCase([100, -100, 100, -100, 100, -100, 100], [100, 100, -100, -100, 100, 100, -100], -10000),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
