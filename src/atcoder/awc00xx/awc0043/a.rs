// https://atcoder.jp/contests/awc0043/tasks/awc0043_a

fn run(_n: usize, m: usize, a: Vec<usize>) -> usize {
    a.into_iter()
        .fold(m, |state, a| {
            state * a / 100
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn awc0043_a() {
        let tests = [
            TestCase(3, 100, vec![80, 50, 60], 24),
            TestCase(4, 7, vec![50, 50, 50, 50], 0),
            TestCase(8, 12345, vec![99, 87, 73, 100, 64, 58, 91, 77], 2017),
            TestCase(20, 10000000000000000, vec![97, 88, 76, 100, 65, 54, 43, 92, 81, 70, 59, 48, 37, 100, 99, 83, 67, 52, 41, 30], 1884542668173),
            TestCase(1, 1, vec![1], 0),
        ];

        for TestCase(n, m, a, expected) in tests {
            assert_eq!(run(n, m, a), expected);
        }
    }
}
