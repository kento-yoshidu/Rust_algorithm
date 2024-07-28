// https://atcoder.jp/contests/abc364/tasks/abc364_c

use itertools::Itertools;
use std::cmp::min;

fn calc(n: usize, i: usize, num: usize, vec: &Vec<usize>, x: usize) -> usize {
    if n == i + 1 {
        n
    } else if num + vec[i] > x {
        i + 1
    } else {
        calc(n, i+1, num+vec[i], vec, x)
    }
}

pub fn run(n: usize, x: usize, y: usize, a: Vec<usize>, b: Vec<usize>) -> usize {
    let vec_a: Vec<usize> = a.into_iter().sorted().rev().collect();
    let vec_b: Vec<usize> = b.into_iter().sorted().rev().collect();

    min(calc(n, 0, 0, &vec_a, x), calc(n, 0, 0, &vec_b, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 7, 18, vec![2, 3, 5, 1], vec![8, 8, 1, 4], 2),
            TestCase(5, 200000000000000, 200000000000000, vec![1, 1, 1, 1, 1], vec![2, 2, 2, 2, 2], 5),
            TestCase(8, 30, 30, vec![1, 2, 3, 4, 5, 6, 7, 8], vec![8, 7, 6, 5, 4, 3, 2, 1], 6),
        ];

        for TestCase(n, x, y, a, b, expected) in tests {
            assert_eq!(run(n, x, y, a, b), expected);
        }
    }
}
