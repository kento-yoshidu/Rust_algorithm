// https://atcoder.jp/contests/abc472/tasks/abc472_b

use std::cmp::min;

fn run(n: usize, l: Vec<isize>) -> isize {
    let mut ans = std::isize::MAX;

    for i in 0..n {
        let mut left = 0;
        let mut right = 0;

        for j in 0..n {
            if i >= j {
                right += l[j];
            } else {
                left += l[j];
            }
        }

        ans = min(ans, (left - right).abs())
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, isize);

    #[test]
    fn abc471_b() {
        let tests = [
            TestCase(4, vec![5, 2, 3, 8], 2),
            TestCase(7, vec![31, 41, 59, 26, 53, 58, 97], 51),
            TestCase(10, vec![67011, 35764, 33042, 24098, 63738, 98760, 17199, 68579, 21812, 45408], 28105),
        ];

        for TestCase(n, l, expected) in tests {
            assert_eq!(run(n, l), expected);
        }
    }
}
