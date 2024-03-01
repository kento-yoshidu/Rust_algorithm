// https://atcoder.jp/contests/abc154/tasks/abc154_d

pub fn run(n: usize, k: usize, p: Vec<usize>) -> f64 {
    let mut ans: f64 = 0.0;

    for i in 0..=n-k {
        let mut sum = 0.0;

        for j in 0..k {
            sum += ((p[i+j] + 1) as f64) / 2.0;
        }

        ans = ans.max(sum);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, f64);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 3, vec![1, 2, 2, 4, 5], 7.0),
            TestCase(4, 1, vec![6, 6, 6, 6], 3.5),
            TestCase(10, 4, vec![17, 13, 13, 12, 15, 20, 10, 13, 17, 11], 32.0),
            TestCase(3, 3, vec![700, 384, 43], 565.0),
            TestCase(1, 1, vec![4], 2.5),
        ];

        for TestCase(n, k, p, expected) in tests {
            assert_eq!(run(n, k, p), expected);
        }
    }
}
