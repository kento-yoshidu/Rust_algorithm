// https://atcoder.jp/contests/abc150/tasks/abc150_c

use itertools::Itertools;

fn run(n: usize, p: Vec<usize>, q: Vec<usize>) -> usize {
    let mut a = 0;
    let mut b = 0;

    for (i, arr) in (1..=n).permutations(n).enumerate() {
        if arr == p {
            a = i;
        }
        if arr == q {
            b = i;
        }
    }

    (b as isize - a as isize).abs() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>, usize);

    #[test]
    fn abc150_c() {
        let tests = [
            TestCase(3, vec![1, 3, 2], vec![3, 1, 2], 3),
            TestCase(8, vec![7, 3, 5, 4, 2, 1, 6, 8], vec![3, 8, 2, 5, 4, 6, 7, 1], 17517),
            TestCase(3, vec![1, 2, 3], vec![1, 2, 3], 0),
            TestCase(7, vec![4, 3, 5, 2, 7, 1, 6], vec![5, 2, 1, 3, 4, 7, 6], 543),
            TestCase(5, vec![2, 5, 3, 4, 1], vec![1, 4, 3, 5, 2], 30),
            TestCase(2, vec![2, 1], vec![1, 2], 1),
            TestCase(8, vec![5, 7, 4, 2, 8, 3, 6, 1], vec![1, 7, 8, 4, 6, 2, 5, 3], 19898),
            TestCase(2, vec![1, 2], vec![2, 1], 1),
            TestCase(4, vec![4, 3, 2, 1], vec![1, 3, 2, 4], 21),
        ];

        for TestCase(n, p, q, expected) in tests {
            assert_eq!(run(n, p, q), expected);
        }
    }
}
