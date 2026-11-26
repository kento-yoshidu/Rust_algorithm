// https://atcoder.jp/contests/awc0047/tasks/awc0047_a

fn run(_n: usize, k: isize, t: Vec<isize>) -> usize {
    t.windows(2)
        .filter(|arr| {
            (arr[0] - arr[1]).abs() >= k
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, isize, Vec<isize>, usize);

    #[test]
    fn awc0046_a() {
        let tests = [
            TestCase(5, 3, vec![10, 12, 15, 14, 20], 2),
            TestCase(4, 10, vec![5, 8, 3, 12], 0),
            TestCase(10, 5, vec![0, -4, 2, 7, 1, 10, 6, -1, 4, 4], 6),
            TestCase(20, 100, vec![0, 50, -60, 40, 140, 139, 239, 100, 0, -100, -50, 49, 149, 300, 201, 101, 1, -98, -198, -97], 13),
            TestCase(2, 1000000000, vec![-1000000000, 0], 1),
        ];

        for TestCase(n, k, t, expected) in tests {
            assert_eq!(run(n, k, t), expected);
        }
    }
}
