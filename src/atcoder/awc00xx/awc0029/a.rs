// https://atcoder.jp/contests/awc0029/tasks/awc0029_a

fn run(_n: usize, p: usize, b: usize, k: usize, c: Vec<usize>) -> usize {
    c.into_iter()
        .map(|c| {
            if c < k {
                p * c
            } else {
                (p + b) * c
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, Vec<usize>, usize);

    #[test]
    fn awc0029_a() {
        let tests = [
            TestCase(3, 100, 50, 3, vec![5, 2, 3], 1400),
            TestCase(3, 200, 100, 10, vec![3, 5, 2], 2000),
            TestCase(10, 500, 300, 15, vec![20, 10, 15, 8, 25, 14, 16, 5, 30, 12], 109300),
            TestCase(20, 1000, 500, 20, vec![25, 18, 31, 20, 10, 15, 22, 28, 19, 30, 5, 12, 21, 27, 16, 8, 24, 31, 14, 20], 535500),
            TestCase(1, 1, 1, 1, vec![1], 2),
        ];

        for TestCase(n, p, b, k, c, expected) in tests {
            assert_eq!(run(n, p, b, k, c), expected);
        }
    }
}
