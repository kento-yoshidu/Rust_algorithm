// https://atcoder.jp/contests/joi2026yo1a/tasks/joi2026_yo1a_c

fn run(_n: usize, a: Vec<usize>, b: Vec<usize>) -> Vec<usize> {
    a.into_iter()
        .zip(b)
        .flat_map(|(x, y)| [x, y])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>, Vec<usize>);

    #[test]
    fn joi2026yo1a() {
        let tests = [
            TestCase(3, vec![3, 1, 4], vec![1, 5, 9], vec![3, 1, 1, 5, 4, 9]),
            TestCase(1, vec![2], vec![3], vec![2, 3]),
            TestCase(6, vec![12, 10, 8, 6, 4, 2], vec![11, 9, 7, 5, 3, 1], vec![12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]),
        ];

        for TestCase(n, a, b, expected) in tests {
            assert_eq!(run(n, a, b), expected);
        }
    }
}
