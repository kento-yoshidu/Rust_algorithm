// https://atcoder.jp/contests/abc212/tasks/abc212_c

use std::cmp::min;
use itertools::Itertools;

fn run(n: usize, m: usize, a: Vec<isize>, b: Vec<isize>) -> isize {
    let a: Vec<isize> = a.into_iter().sorted().collect();
    let b: Vec<isize> = b.into_iter().sorted().collect();

    let mut ans = std::isize::MAX;

    let mut x = 0;
    let mut y = 0;

    loop {
        if x == n || y == m {
            break;
        }

        ans = min(ans, (a[x] - b[y]).abs());

        if a[x] > b[y] {
            y += 1;
        } else {
            x += 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<isize>, Vec<isize>, isize);

    #[test]
    fn abc212_c() {
        let tests = [
            TestCase(2, 2, vec![1, 6], vec![4, 9], 2),
            TestCase(1, 1, vec![10], vec![10], 0),
            TestCase(6, 8, vec![82, 76, 82, 82, 71, 70], vec![17, 39, 67, 2, 45, 35, 22, 24], 3),
        ];

        for TestCase(n, m, a, b, expected) in tests {
            assert_eq!(run(n, m, a, b), expected);
        }
    }
}
