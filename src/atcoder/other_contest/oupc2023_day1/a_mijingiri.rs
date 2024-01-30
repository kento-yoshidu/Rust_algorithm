// https://atcoder.jp/contests/oupc2023-day1/tasks/oupc2023_day1_a

pub fn run(a: usize, t: usize) -> usize {
    (a - 1) / t
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(7, 3, 2),
            TestCase(6, 3, 1),
            TestCase(100, 1, 99)
        ];

        for TestCase(a, t, expected) in tests {
            assert_eq!(expected, run(a, t));
        }
    }
}
