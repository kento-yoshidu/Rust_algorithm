// https://atcoder.jp/contests/joi2026yo1c/tasks/joi2026_yo1c_d

fn run(_n: usize, _m: usize, a: Vec<Vec<usize>>) -> usize {
    a.into_iter()
        .map(|v| {
            v.into_iter().max().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<Vec<usize>>, usize);

    #[test]
    fn joi2026yo1c_d() {
        let tests = [
            TestCase(3, 2, vec![vec![4, 7], vec![9, 6], vec![5, 5]], 21),
            TestCase(1, 5, vec![vec![2, 8, 3, 6, 1]], 8),
            TestCase(4, 1, vec![vec![6], vec![9], vec![1], vec![8]], 24),
            TestCase(5, 6, vec![ vec![91, 30, 75, 5, 81, 16], vec![82, 26, 5, 76, 91, 94], vec![36, 38, 44, 81, 43, 65], vec![4, 63, 68, 14, 100, 27], vec![8, 54, 17, 36, 64, 87]], 453),
        ];

        for TestCase(n, m, a, expected) in tests {
            assert_eq!(run(n, m, a), expected);
        }
    }
}
