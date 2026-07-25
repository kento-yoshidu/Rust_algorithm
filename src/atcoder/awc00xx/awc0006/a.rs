// https://atcoder.jp/contests/awc0006/tasks/awc0006_a

fn run(_n: usize, l: usize, w: usize, d: Vec<usize>) -> usize {
    d.into_iter()
        .filter(|d| l - w <= *d && *d <= l + w)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, usize);

    #[test]
    fn awc0006_a() {
        let tests = [
            TestCase(5, 10, 3, vec![5, 8, 10, 13, 15], 3),
            TestCase(4, 50, 5, vec![1, 10, 30, 100], 0),
            TestCase(10, 100, 20, vec![50, 79, 80, 95, 100, 120, 121, 150, 200, 60], 4),
            TestCase(20, 500000000, 100000000, vec![1, 100000000, 399999999, 400000000, 400000001, 450000000, 500000000, 550000000, 599999999, 600000000, 600000001, 700000000, 800000000, 999999999, 1000000000, 250000000, 123456789, 500000001, 499999999, 350000000], 9),
            TestCase(1, 1, 1, vec![1], 1),
        ];

        for TestCase(n, l, w, d, expected) in tests {
            assert_eq!(run(n, l, w, d), expected);
        }
    }
}
