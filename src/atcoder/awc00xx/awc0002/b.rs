// https://atcoder.jp/contests/awc0002/tasks/awc0002_b

fn run(_n: usize, _m: usize, k: usize, a: Vec<usize>, b: Vec<usize>) -> (usize, usize) {
    b.into_iter()
        .filter(|i| a[i-1] < k)
        .fold((0, 0), |(count, sum), i| (count + 1, sum + a[i-1]))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, Vec<usize>, (usize, usize));

    #[test]
    fn awc0002_b() {
        let tests = [
            TestCase(5, 3, 10, vec![8, 12, 5, 15, 7], vec![1, 3, 4], (2, 13)),
            TestCase(8, 5, 20, vec![25, 18, 30, 12, 8, 22, 15, 19], vec![2, 4, 5, 6, 8], (4, 57)),
            TestCase(15, 7, 100, vec![150, 80, 95, 200, 45, 120, 75, 180, 90, 60, 110, 55, 130, 85, 70], vec![3, 5, 7, 9, 10, 12, 14], (7, 505)),
        ];

        for TestCase(n, m, k, a, b, expected) in tests {
            assert_eq!(run(n, m, k, a, b), expected);
        }
    }
}
