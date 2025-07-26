// https://atcoder.jp/contests/abc414/tasks/abc414_a

fn run(_n: usize, l: usize, r: usize, xy: Vec<(usize, usize)>) -> usize {
    xy.into_iter()
        .filter(|(x, y)| *x <= l && r <= *y)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<(usize, usize)>, usize);

    #[test]
    fn abc414_a() {
        let tests = [
            TestCase(5, 19, 22, vec![(17, 23), (20, 23), (19, 22), (0, 23), (12, 20)], 3),
            TestCase(3, 12, 13, vec![(0, 1), (0, 1), (0, 1)], 0),
            TestCase(10, 8, 14, vec![(5, 20), (14, 21), (9, 21), (5, 23), (8, 10), (0, 14), (3, 8), (2, 6), (0, 16), (5, 20)], 5),
        ];

        for TestCase(n, l, r, xy, expected) in tests {
            assert_eq!(run(n, l, r, xy), expected);
        }
    }
}
