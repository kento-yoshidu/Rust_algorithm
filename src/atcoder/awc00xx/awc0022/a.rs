// https://atcoder.jp/contests/awc0022/tasks/awc0022_a

fn run(_n: usize, d: usize, k: usize, w: Vec<usize>) -> usize {
    w.into_iter()
        .filter(|w| *w > d * k)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, usize);

    #[test]
    fn awc0022_a() {
        let tests = [
            TestCase(3, 10, 5, vec![100, 50, 30], 1),
            TestCase(5, 15, 3, vec![80, 45, 46, 20, 100], 3),
            TestCase(10, 7, 10, vec![100, 70, 71, 50, 35, 28, 1, 72, 99, 65], 4),
        ];

        for TestCase(n, d, k, w, expected) in tests {
            assert_eq!(run(n, d, k, w), expected);
        }
    }
}
