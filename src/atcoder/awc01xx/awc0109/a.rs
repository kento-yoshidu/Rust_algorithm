// https://atcoder.jp/contests/awc0109/tasks/awc0109_a

fn run(n: usize, s: usize, k: usize, a: Vec<usize>) -> usize {
    a.into_iter().sum::<usize>() + s + n * k
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, usize);

    #[test]
    fn awc0109_a() {
        let tests = [
            TestCase(3, 2, 1, vec![3, 1, 2], 11),
            TestCase(1, 5, 3, vec![4], 12),
            TestCase(5, 1000000000, 1000000000, vec![1000000000, 1000000000, 1000000000, 1000000000, 1000000000], 11000000000),
        ];

        for TestCase(n, s, k, a, expected) in tests {
            assert_eq!(run(n, s, k, a), expected);
        }
    }
}
