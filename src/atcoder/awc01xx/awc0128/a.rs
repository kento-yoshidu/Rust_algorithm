// https://atcoder.jp/contests/awc0128/tasks/awc0128_a

fn run(_n: usize, k: isize, a: Vec<isize>) -> usize {
    a.into_iter()
        .filter(|a| a % k == 0)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, isize, Vec<isize>, usize);

    #[test]
    fn awc0128_a() {
        let tests = [
            TestCase(5, 3, vec![1, 3, 6, -4, 0], 3),
            TestCase(6, 5, vec![1, 2, 3, 4, 6, 7], 0),
            TestCase(12, 7, vec![14, -21, 28, 5, 0, 49, 56, -1, 70, 100, -98, 13], 8),
            TestCase(30, 12, vec![12, 24, 36, 48, 60, 72, 84, 96, 108, 120, -12, -24, -36, -48, -60, -72, 1, 11, 13, 25, 37, 49, 61, 73, 85, 97, 0, 144, -120, 1000000000], 19),
            TestCase(1, 1000000000, vec![-1000000000], 1),
        ];

        for TestCase(n, k, a, expected) in tests {
            assert_eq!(run(n, k, a), expected)
        }
    }
}
