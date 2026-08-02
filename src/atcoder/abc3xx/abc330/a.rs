// https://atcoder.jp/contests/abc330/tasks/abc330_a

fn run(_n: usize, l: usize, a: Vec<usize>) -> usize {
    a.into_iter()
        .filter(|&num| {
            num >= l
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn abc330_a() {
        let tests = [
            TestCase(5, 60, vec![60, 20, 100, 90, 40], 3),
            TestCase(4, 80, vec![79, 78, 77, 76], 0),
            TestCase(10, 50, vec![31, 41, 59, 26, 53, 58, 97, 93, 23, 84], 6),
        ];

        for TestCase(n, l, a, expected) in tests {
            assert_eq!(run(n, l, a), expected);
        }
    }
}
