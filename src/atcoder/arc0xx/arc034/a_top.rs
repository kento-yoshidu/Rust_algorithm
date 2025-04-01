// https://atcoder.jp/contests/arc034/tasks/arc034_1

fn run(_n: usize, a: Vec<[usize; 5]>) -> f64 {
    a.into_iter()
        .map(|arr| -> f64 {
            let sum: f64 = arr[0..4].into_iter().map(|n| *n as f64).sum();
            sum + arr[4] as f64 * 110.0 / 900.0
        })
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<[usize; 5]>, f64);

    #[test]
    fn test() {
        let tests = [
                TestCase(2, vec![[37, 54, 68, 66, 802], [58, 108, 106, 103, 871]], 481.4555555555555),
                TestCase(2, vec![[80, 120, 120, 120, 900], [0, 0, 0, 0, 731]], 550.0),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
