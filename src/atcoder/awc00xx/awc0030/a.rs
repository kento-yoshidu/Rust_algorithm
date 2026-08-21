// https://atcoder.jp/contests/awc0030/tasks/awc0030_a

fn run(_n: usize, m: usize, a: Vec<usize>) -> Vec<(usize, usize)> {
    a.into_iter()
        .map(|a| {
            (a / m, a % m)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<(usize, usize)>);

    #[test]
    fn awc0030_a() {
        let tests = [
            TestCase(3, 4, vec![10, 7, 3], vec![(2, 2), (1, 3), (0, 3)]),
            TestCase(5, 3, vec![0, 1, 2, 3, 6], vec![(0, 0), (0, 1), (0, 2), (1, 0), (2, 0)]),
            TestCase(8, 7, vec![100, 49, 50, 0, 35, 1, 999, 123], vec![(14, 2), (7, 0), (7, 1), (0, 0), (5, 0), (0, 1), (142, 5), (17, 4)]),
            TestCase(10, 1000000000, vec![999999999, 1000000000, 500000000, 0, 1, 999999998, 123456789, 987654321, 2000000, 750000000], vec![(0, 999999999), (1, 0), (0, 500000000), (0, 0), (0, 1), (0, 999999998), (0, 123456789), (0, 987654321), (0, 2000000), (0, 750000000), ]),
            TestCase(1, 1, vec![0], vec![(0, 0)]),
        ];

        for TestCase(n, m, a, expected) in tests {
            assert_eq!(run(n, m, a), expected);
        }
    }
}
