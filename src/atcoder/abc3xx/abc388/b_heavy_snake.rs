// https://atcoder.jp/contests/abc388/tasks/abc388_b

fn run(_n: usize, d: usize, tl: Vec<(usize, usize)>) -> Vec<usize> {
    (1..=d)
        .flat_map(|x| {
            let max = tl.iter()
                .map(|(t, l)| {
                    t * (l + x)
                })
                .max()
                .unwrap();

            Some(max)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 3, vec![(3, 3), (5, 1), (2, 4), (1, 10)], vec![12, 15, 20]),
            TestCase(1, 4, vec![(100, 100)], vec![10100, 10200, 10300, 10400]),
        ];

        for TestCase(n, d, tl, expected) in tests {
            assert_eq!(run(n, d, tl), expected);
        }
    }
}
