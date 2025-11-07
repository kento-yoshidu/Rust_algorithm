// https://atcoder.jp/contests/joi2026yo1b/tasks/joi2026_yo1b_d

use std::cmp::{min, max};

fn run(_n: usize, c: Vec<usize>) -> Vec<usize> {
    let mut ans = Vec::new();

    for (i, num) in c.iter().enumerate() {
        let mut count = 0;

        for (j, num2) in c.iter().enumerate() {
            if num == num2 {
                count += max(i, j) - min(i, j);
            }
        }

        ans.push(count);
    }

    ans
}


#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>);

    #[test]
    fn joi2026yo1b_d() {
        let tests = [
            TestCase(4, vec![1, 2, 1, 1], vec![5, 0, 3, 4]),
            TestCase(1, vec![100], vec![0]),
            TestCase(8, vec![1, 3, 9, 7, 3, 3, 7, 9], vec![0, 7, 5, 3, 4, 5, 3, 5]),
            TestCase(10, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![ 45, 37, 31, 27, 25, 25, 27, 31, 37, 45]),
        ];

        for TestCase(n, c, expected) in tests {
            assert_eq!(run(n, c), expected);
        }
    }
}
