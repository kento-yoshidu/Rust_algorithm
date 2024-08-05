// https://atcoder.jp/contests/code-formula-2014-qualb/tasks/code_formula_2014_qualB_a

fn run(a: usize) -> usize {
    7 - a
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, 1),
            TestCase(3, 4),
        ];

        for TestCase(a, expected) in tests {
            assert_eq!(run(a), expected);
        }
    }
}
