// https://atcoder.jp/contests/abc344/tasks/abc344_b

fn run(a: Vec<usize>) -> Vec<usize> {
    a.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<usize>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(vec![3, 2, 1, 0], vec![0, 1, 2, 3]),
        ];

        for TestCase(a, expected) in tests {
            assert_eq!(run(a), expected);
        }
    }
}
