// https://atcoder.jp/contests/abc005/tasks/abc005_2

fn run(_n: usize, t: Vec<usize>) -> usize {
    t.into_iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc005_b() {
        let tests = [
            TestCase(3, vec![1, 2, 3], 1),
            TestCase(3, vec![3, 3, 3], 3),
            TestCase(5, vec![3, 1, 4, 1, 5], 1),
        ];

        for TestCase(n, t, expected) in tests {
            assert_eq!(run(n, t), expected);
        }
    }
}
