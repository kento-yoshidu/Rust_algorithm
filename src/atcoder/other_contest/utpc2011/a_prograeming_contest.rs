// https://atcoder.jp/contests/utpc2011/tasks/utpc2011_1

pub fn run(_m: usize, _n: usize, a: Vec<Vec<usize>>) -> usize {
    a.iter()
        .map(|arr| {
            arr.iter().sum()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<Vec<usize>>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 4, vec![vec![1, 0, 1, 0], vec![1, 1, 1, 0], vec![0, 0, 0, 1]], 3),
            TestCase(3, 4, vec![vec![1, 1, 1, 1], vec![1, 1, 1, 1], vec![1, 1, 1, 1]], 4),
            TestCase(1, 1, vec![vec![0]], 0),
        ];

        for TestCase(m, n, a, expected) in tests {
            assert_eq!(run(m, n, a), expected);
        }
    }
}
