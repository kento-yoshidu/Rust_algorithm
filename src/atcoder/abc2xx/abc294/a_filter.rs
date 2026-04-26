// https://atcoder.jp/contests/abc294/tasks/abc294_a

fn run(_n: usize, a: Vec<usize>) -> Vec<usize> {
    a.into_iter()
        .filter(|i| i % 2 == 0)
        .collect::<Vec<usize>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>);

    #[test]
    fn abc294_a() {
        let tests = [
            TestCase(5, vec![1, 2, 3, 5, 6], vec![2, 6]),
            TestCase(5, vec![2, 2, 2, 3, 3], vec![2, 2, 2]),
            TestCase(10, vec![22, 3, 17, 8, 30, 15, 12, 14, 11, 17], vec![22, 8, 30, 12, 14]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
