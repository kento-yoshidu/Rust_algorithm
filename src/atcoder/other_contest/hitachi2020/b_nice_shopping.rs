// https://atcoder.jp/contests/hitachi2020/tasks/hitachi2020_b

use std::cmp::min;

fn run(_a: usize, _b: usize, _m: usize, a_vec: Vec<usize>, b_vec: Vec<usize>, vec: Vec<(usize, usize, usize)>) -> usize {
    let non_discount = a_vec.iter().min().unwrap() + b_vec.iter().min().unwrap();

    let mut ans = std::usize::MAX;

    for (x, y, c) in vec.iter() {
        ans = ans.min(a_vec[x-1] + b_vec[y-1] - c);
    }

    min(ans, non_discount)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, Vec<usize>, Vec<(usize, usize, usize)>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 3, 1, vec![3, 3], vec![3, 3, 3], vec![(1, 2, 1)], 5),
            TestCase(1, 1, 2, vec![10], vec![10], vec![(1, 1, 5), (1, 1, 10)], 10),
            TestCase(2, 2, 1, vec![3, 5], vec![3, 5], vec![(2, 2, 2)], 6),
        ];

        for TestCase(a, b, m, a_vec, b_vec, vec, expected) in tests {
            assert_eq!(run(a, b, m, a_vec, b_vec, vec), expected);
        }
    }
}
