// https://atcoder.jp/contests/abc468/tasks/abc468_a

fn run(_n: usize, a: Vec<usize>) -> usize {
    a.windows(3)
        .filter(|arr| arr[0] < arr[1] && arr[1] > arr[2])
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc468_a() {
        let tests = [
            TestCase(6, vec![3, 1, 4, 1, 5, 2], 2),
            TestCase(5, vec![1, 1, 1, 2, 1], 1),
            TestCase(10, vec![7, 3, 9, 8, 10, 3, 1, 5, 5, 4], 2),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
