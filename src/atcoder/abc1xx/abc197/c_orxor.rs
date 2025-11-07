// https://atcoder.jp/contests/abc197/tasks/abc197_c

use std::cmp::min;

fn run(n: usize, a: Vec<usize>) -> usize {
    let mut ans = std::usize::MAX;

    for bit in 0..(1 << n) {
        let mut xor = 0;
        let mut oor = 0;

        for i in 0..n {
            oor |= a[i];

            if (bit >> i) & 1 != 0 || i == n-1 {
                xor ^= oor;
                oor = 0;
            }
        }

        ans = min(ans, xor);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc197_c() {
        let tests = [
            TestCase(3, vec![1, 5, 7], 2),
            TestCase(3, vec![10, 10, 10], 0),
            TestCase(4, vec![1, 3, 3, 1], 0),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
