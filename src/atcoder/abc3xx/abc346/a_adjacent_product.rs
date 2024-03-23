// https://atcoder.jp/contests/abc346/tasks/abc346_a

fn run(_n: usize, a: Vec<usize>) -> Vec<usize> {
    a.windows(2)
        .map(|v| {
            v[0] * v[1]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![3, 4, 6], vec![12, 24]),
            TestCase(5, vec![22, 75, 26, 45, 72], vec![1650, 1950, 1170, 3240]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
