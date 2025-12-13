// https://atcoder.jp/contests/abc191/tasks/abc191_b

fn run(_n: usize, x: usize, a: Vec<usize>) -> Vec<usize> {
    a.into_iter()
        .filter(|num| {
            *num != x
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>);

    #[test]
    fn abc191_b() {
        let tests = [
            TestCase(5, 5, vec![3, 5, 6, 5, 4], vec![3, 6, 4]),
            TestCase(3, 3, vec![3, 3, 3], Vec::new()),
        ];

        for TestCase(n, x, a, expected) in tests {
            assert_eq!(run(n, x, a), expected);
        }
    }
}
