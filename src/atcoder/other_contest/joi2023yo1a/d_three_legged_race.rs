// https://atcoder.jp/contests/joi2023yo1a/tasks/joi2023_yo1a_d

fn run(_n: usize, a: Vec<usize>) -> usize {
    a.into_iter()
        .fold(0, |acc, x| acc ^ x)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![1, 4, 2, 1, 3, 4, 3], 2),
            TestCase(10, vec![5, 7, 1, 9, 8, 8, 2, 9, 6, 5, 1, 3, 6, 4, 7, 3, 10, 2, 4], 10),
            TestCase(1, vec![1], 1),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
