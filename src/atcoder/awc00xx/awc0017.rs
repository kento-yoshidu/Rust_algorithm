// https://atcoder.jp/contests/awc0017/tasks/awc0017_a

fn run(_n: usize, k: usize, cd: Vec<(usize, usize)>) -> usize {
    cd.into_iter()
        .filter_map(|(c, d)| {
            (c <= k).then_some(d)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, usize);

    #[test]
    fn awc0017_a() {
        let tests = [
            TestCase(5, 2, vec![(1, 10), (3, 20), (0, 15), (2, 30), (5, 25)], 55),
            TestCase(4, 1, vec![(5, 100), (3, 200), (10, 50), (2, 300)], 0),
            TestCase(10, 5, vec![(3, 120), (7, 250), (5, 80), (0, 300), (12, 60), (4, 150), (6, 90), (1, 200), (9, 110), (5, 170)], 1020),
            TestCase(20, 100, vec![(50, 1000000000), (101, 500000000), (99, 800000000), (200, 300000000), (0, 999999999), (100, 750000000), (150, 600000000), (30, 450000000), (75, 200000000), (110, 350000000), (1, 900000000), (88, 100000000), (100, 550000000), (250, 400000000), (99, 650000000), (1000000000, 1000000000), (45, 700000000), (102, 850000000), (100, 500000000), (60, 300000000)], 7899999999),
            TestCase(1, 0, vec![(0, 1)], 1),
        ];

        for TestCase(n, k, cd, expected) in tests {
            assert_eq!(run(n, k, cd), expected);
        }
    }
}
