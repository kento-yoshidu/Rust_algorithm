// https://atcoder.jp/contests/abc343/tasks/abc343_b

fn run(_n: usize, a: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    a.iter()
        .map(|v| {
            v.iter()
                .enumerate()
                .filter(|(_, num)| {
                    **num == 1
                })
                .map(|(i, _)| {
                    i+1
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<Vec<usize>>, Vec<Vec<usize>>);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![vec![0, 1, 1, 0], vec![1, 0, 0, 1], vec![1, 0, 0, 0], vec![0, 1, 0, 0]], vec![vec![2, 3], vec![1, 4], vec![1], vec![2]]),
            TestCase(2, vec![vec![0, 0], vec![0, 0]], vec![vec![], vec![]]),
            TestCase(5, vec![vec![0, 1, 0, 1, 1], vec![1, 0, 0, 1, 0], vec![0, 0, 0, 0, 1], vec![1, 1, 0, 0, 1], vec![1, 0, 1, 1, 0]], vec![vec![2, 4, 5], vec![1, 4], vec![5], vec![1, 2, 5], vec![1, 3, 4]]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
