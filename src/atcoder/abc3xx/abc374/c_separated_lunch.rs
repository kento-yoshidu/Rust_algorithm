// https://atcoder.jp/contests/abc374/tasks/abc374_c

use std::cmp::{min, max};

fn run(n: usize, k: Vec<usize>) -> usize {
    let mut ans = std::usize::MAX;

    for bit in 0..(1 << n) {
        let mut left = 0;
        let mut right = 0;

        for i in 0..n {
            if bit & 1 << i != 0 {
                left += k[i];
            } else {
                right += k[i];
            }
        }

        ans = min(ans, max(left ,right));
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, vec![2, 3, 5, 10, 12], 17),
            TestCase(2, vec![1, 1], 1),
            TestCase(6, vec![22, 25, 26, 45, 22, 31], 89),
        ];

        for TestCase(n, k, expected) in tests {
            assert_eq!(run(n, k), expected);
        }
    }
}
