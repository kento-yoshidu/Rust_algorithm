// https://atcoder.jp/contests/abc347/tasks/abc347_a

fn run(_n: usize, k: usize, a: Vec<usize>) -> Vec<usize> {
    a.into_iter()
        .filter(|num| {
            num % k == 0
        })
        .map(|num| {
            num / k
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 2, vec![2, 5, 6, 7, 10], vec![1, 3, 5]),
            TestCase(3, 1, vec![3, 4, 7], vec![3, 4, 7]),
            TestCase(5, 10, vec![50, 51, 54, 60, 65], vec![5, 6]),
        ];

        for TestCase(n, k, a, expected) in tests {
            assert_eq!(run(n, k, a), expected);
        }
    }
}
