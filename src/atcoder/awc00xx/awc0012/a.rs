// https://atcoder.jp/contests/awc0012/tasks/awc0012_a

fn run(_n: usize, t: usize, h: Vec<usize>, c: Vec<usize>) -> usize {
    h.into_iter()
        .zip(c)
        .filter_map(|(h, c)| {
            (h <= t).then_some(c)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>, usize);

    #[test]
    fn awc0012_a() {
        let tests = [
            TestCase(5, 3, vec![1, 5, 3, 4, 2], vec![10, 20, 30, 40, 50], 90),
            TestCase(4, 2, vec![5, 10, 8, 3], vec![100, 200, 300, 400], 0),
            TestCase(10, 50, vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], vec![5, 15, 25, 35, 45, 55, 65, 75, 85, 95], 125),
            TestCase(20, 500, vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 150, 250, 350, 450, 550, 650, 750, 850, 950, 50], vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 15, 25, 35, 45, 55, 65, 75, 85, 95, 5], 275),
            TestCase(1, 1000000000, vec![1000000000], vec![999999999], 999999999),
        ];

        for TestCase(n, t, h, c, expected) in tests {
            assert_eq!(run(n, t, h, c), expected);
        }
    }
}
