// https://atcoder.jp/contests/abc290/tasks/abc290_a

fn run(_n: usize, m: usize, a: &Vec<usize>, b: &Vec<usize>) -> usize {
    let mut ans = 0;

    for i in 0..m {
        ans += a[b[i] - 1]
    }

    ans
}

fn run2(_n: usize, _m: usize, a: &Vec<usize>, b: &Vec<usize>) -> usize {
    b.into_iter().fold(0, |sum, i| {
        sum + a[i - 1]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 2, vec![10, 20, 30], vec![1, 3], 40),
            TestCase(4, 1, vec![1, 1, 1, 100], vec![4], 100),
            TestCase(8, 4, vec![22, 75, 26, 45, 72, 81, 47, 29], vec![4, 6, 7, 8], 202),
        ];

        for TestCase(n, m, a, b, expected) in tests {
            assert_eq!(run(n, m, &a, &b), expected);
            assert_eq!(run2(n, m, &a, &b), expected);
        }
    }
}
