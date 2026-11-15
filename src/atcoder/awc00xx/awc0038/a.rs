// https://atcoder.jp/contests/awc0038/tasks/awc0038_a

fn run(_n: usize, h: Vec<usize>) -> Vec<usize> {
    h.into_iter()
        .enumerate()
        .scan(0, |max, (i, h)| {
            Some((h > *max).then(|| { *max = h; i + 1 }))
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>);

    #[test]
    fn awc0038_a() {
        let tests = [
            TestCase(5, vec![3, 1, 4, 1, 5], vec![1, 3, 5]),
            TestCase(8, vec![10, 20, 15, 25, 5, 30, 28, 35], vec![1, 2, 4, 6, 8]),
            TestCase(15, vec![100, 50, 80, 120, 110, 130, 125, 140, 200, 150, 180, 250, 240, 300, 299], vec![1, 4, 6, 8, 9, 12, 14]),
        ];

        for TestCase(n, h, expected) in tests {
            assert_eq!(run(n, h), expected);
        }
    }
}
