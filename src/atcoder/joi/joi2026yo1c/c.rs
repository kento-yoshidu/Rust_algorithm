// https://atcoder.jp/contests/joi2026yo1c/tasks/joi2026_yo1c_c

fn run(_n: usize, x: usize, h: Vec<usize>) -> usize {
    h.into_iter()
        .filter(|h| *h >= x)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn joi2026yo1c_c() {
        let tests = [
            TestCase(3, 20, vec![18, 25, 20], 2),
            TestCase(4, 10, vec![3, 9, 1, 3], 0),
            TestCase(1, 100, vec![100], 1),
            TestCase(5, 634, vec![829, 679, 632, 601, 600], 2),
        ];

        for TestCase(n, x, h, expected) in tests {
            assert_eq!(run(n, x, h), expected);
        }
    }
}
