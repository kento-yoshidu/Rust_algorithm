// https://atcoder.jp/contests/abc181/tasks/abc181_b

fn run(_n: usize, ab: Vec<(usize, usize)>) -> usize {
    ab.into_iter()
        .map(|(l, r)| {
            (l..=r).sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, usize);

    #[test]
    fn abc181_b() {
        let tests = [
            TestCase(5, vec![(1, 3), (3, 5)], 18),
            TestCase(3, vec![(11, 13), (17, 47), (359, 44683)], 998244353),
            TestCase(1, vec![(1, 1000000)], 500000500000),
        ];

        for TestCase(n, ab, expected) in tests {
            assert_eq!(run(n, ab), expected);
        }
    }
}
