// https://atcoder.jp/contests/abc383/tasks/abc383_a

use std::cmp::max;

fn run(n: usize, tv: Vec<(i32, i32)>) -> i32 {
    let mut cur = 0;

    for i in 0..n {
        if i == 0 {
            cur += tv[0].1
        } else {
            cur = max(0, cur - (tv[i].0 - tv[i-1].0));
            cur += tv[i].1;
        }
    }

    cur
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(i32, i32)>, i32);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![(1, 3), (3, 1), (4, 4), (7, 1)], 3),
            TestCase(3, vec![(1, 8), (10, 11), (21, 5)], 5),
            TestCase(10, vec![(2, 1), (22, 10), (26, 17), (29, 2), (45, 20), (47, 32), (72, 12), (75, 1), (81, 31), (97, 7)], 57),
        ];

        for TestCase(n, tv, expected) in tests {
            assert_eq!(run(n, tv), expected);
        }
    }
}