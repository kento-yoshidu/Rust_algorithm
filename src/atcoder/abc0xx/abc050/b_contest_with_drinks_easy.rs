// https://atcoder.jp/contests/abc050/tasks/abc050_b

fn run(_n: usize, t: Vec<usize>, _m : usize, p: Vec<(usize, usize)>) -> Vec<usize> {
    let sum: usize = t.iter().sum();

    p.iter().map(|tup| {
        sum - t[tup.0-1] + tup.1
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize, Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![2, 1, 4], 2, vec![(1, 1), (2, 3)], vec![6, 9]),
            TestCase(5, vec![7, 2, 3, 8, 5], 3, vec![(4, 2), (1, 7), (4, 13)], vec![19, 25, 30]),
        ];

        for TestCase(n, t, m, p, expected) in tests {
            assert_eq!(run(n, t, m, p), expected);
        }
    }
}
