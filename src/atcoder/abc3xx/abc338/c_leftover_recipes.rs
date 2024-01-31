// https://atcoder.jp/contests/abc338/tasks/abc338_c

use std::cmp::{min, max};

pub fn run(n: usize, q: Vec<usize>, a: Vec<usize>, b: Vec<usize>) -> usize {
    let mut a_max = std::usize::MAX;

    for i in 0..n {
        if a[i] > 0 {
            a_max = min(a_max, q[i] / a[i]);
        }
    }

    let mut ans = 0;

    for a_n in 0..=a_max {
        let mut b_count = std::usize::MAX;

        for i in 0..n {
            // aをa_n個分作った時、qが何グラム残るか
            let rest = q[i] - a[i]*a_n;

            if b[i] > 0 {
                b_count = min(b_count, rest / b[i]);
            }
        }

        ans = max(ans, a_n + b_count);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, vec![800, 300], vec![100, 100], vec![200, 10], 5),
            TestCase(2, vec![800, 300], vec![100, 0], vec![0, 10], 38),
            TestCase(2, vec![800, 300], vec![801, 300], vec![800, 301], 0),
            TestCase(10, vec![1000000, 1000000, 1000000, 1000000, 1000000, 1000000, 1000000, 1000000, 1000000, 1000000], vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0], 222222),
        ];

        for TestCase(n, q, a, b, expected) in tests {
            assert_eq!(run(n, q, a, b), expected);
        }
    }
}
