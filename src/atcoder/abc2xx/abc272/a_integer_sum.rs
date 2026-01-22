// https://atcoder.jp/contests/abc272/tasks/abc272_a

fn run(_n: usize, a: Vec<usize>) -> usize {
    a.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc272_a() {
        let tests = [
            TestCase(3, vec![2, 7, 2], 11),
            TestCase(1, vec![3], 3),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
