// https://atcoder.jp/contests/indeednow-quala/tasks/indeednow_2015_quala_1

fn run(a: usize, b: usize) -> usize {
    a.to_string().len() * b.to_string().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(123, 12, 6),
            TestCase(10, 1000, 8),
            TestCase(2, 3, 1),
            TestCase(1000000, 1000000, 49),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
