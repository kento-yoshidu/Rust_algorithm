// https://atcoder.jp/contests/awc0127/tasks/awc0127_a

fn run(_n: usize, k: usize, s: Vec<isize>) -> f64 {
    match s.into_iter().fold((0, 0), |(sum, count), s| {
        (k as isize <= s)
            .then_some(s)
            .map_or((sum, count), |s| (sum + s, count + 1))
    }) {
        (_, 0) => -1.0,
        (sum, count) => sum as f64 / count as f64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<isize>, f64);

    #[test]
    fn awc0127_a() {
        let tests = [
            TestCase(5, 8, vec![3, 10, 7, 12, 9], 10.333333333333334),
            TestCase(4, 15, vec![3, 7, 10, 14], -1.0),
            TestCase(10, 50, vec![45, 60, 55, 30, 70, 48, 52, 49, 80, 51], 61.333333333333336),
            TestCase(20, 500, vec![120, 600, 450, 800, 300, 550, 499, 501, 750, 200, 630, 480, 520, 999, 100, 500, 350, 680, 490, 510],640.0),
            TestCase(1, 1000000000, vec![1000000000], 1000000000.0),
        ];

        for TestCase(n, k, s, expected) in tests {
            assert_eq!(run(n, k, s), expected);
        }
    }
}
