// https://atcoder.jp/contests/abc340/tasks/abc340_a

pub fn run(a: usize, b: usize, d: usize) -> Vec<usize> {
    (a..=b)
        .step_by(d)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 9, 2, vec![3, 5, 7, 9]),
            TestCase(10, 10, 1, vec![10]),
        ];

        for TestCase(a, b, d, expected) in tests {
            assert_eq!(run(a, b, d), expected);
        }
    }
}
