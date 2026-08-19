// https://atcoder.jp/contests/awc0002/tasks/awc0002_a

fn run(_n: usize, k: isize, a: Vec<isize>) -> isize {
    a.into_iter()
        .position(|n| n == k)
        .map_or(-1, |i| i as isize + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, isize, Vec<isize>, isize);

    #[test]
    fn awc0002_a() {
        let tests = [
            TestCase(5, 3, vec![1, 4, 3, 2, 3], 3),
            TestCase(7, 100, vec![50, 200, 100, 100, 300, 100, 400], 3),
            TestCase(10, 999999999, vec![1, 1000000000, 500000000, 123456789, 999999999, 999999999, 777777777, 888888888, 999999999, 1], 5),
        ];

        for TestCase(n, k, a, expected) in tests {
            assert_eq!(run(n, k, a), expected);
        }
    }
}
