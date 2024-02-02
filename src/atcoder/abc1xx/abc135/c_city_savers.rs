// https://atcoder.jp/contests/abc135/tasks/abc135_c

use std::cmp::min;

pub fn run(n: usize, a: Vec<usize>, b: Vec<usize>) -> usize {
    let mut ans = 0;

    let mut vec_a = a.clone();
    let mut vec_b = b.clone();

    for i in 0..n {
        let rest_1 = min(vec_a[i], vec_b[i]);

        ans += rest_1;
        vec_a[i] -= rest_1;
        vec_b[i] -= rest_1;

        let rest_2 = min(vec_a[i+1], vec_b[i]);

        ans += rest_2;
        vec_a[i+1] -= rest_2;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, vec![3, 5, 2], vec![4, 5], 9),
            TestCase(3, vec![5, 6, 3, 8], vec![5, 100, 8], 22),
            TestCase(2, vec![100, 1, 1], vec![1, 100], 3),
            TestCase(1, vec![1, 1], vec![100], 2),

        ];

        for TestCase(n, a, b, expected) in tests {
            assert_eq!(run(n, a, b), expected);
        }
    }
}
