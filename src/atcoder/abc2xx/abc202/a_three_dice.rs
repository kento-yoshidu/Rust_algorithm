// https://atcoder.jp/contests/abc202/tasks/abc202_a

fn run(a: usize, b: usize, c: usize) -> usize {
    21 - (a + b + c)
}

fn run2(a: usize, b: usize, c: usize) -> usize {
    [a, b, c].into_iter()
        .map(|num| 7 - num)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn abc202_a() {
        let tests = [
            TestCase(1, 4, 3, 13),
            TestCase(5, 6, 4, 6),
            TestCase(6, 6, 4, 5),
            TestCase(4, 3, 2, 12),
            TestCase(6, 1, 1, 13),
            TestCase(4, 1, 5, 11),
            TestCase(6, 1, 5, 9),
            TestCase(4, 6, 4, 7),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
            assert_eq!(run2(a, b, c), expected);
        }
    }
}
