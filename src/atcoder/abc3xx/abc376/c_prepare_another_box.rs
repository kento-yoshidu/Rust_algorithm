// https://atcoder.jp/contests/abc376/tasks/abc376_c

use itertools::Itertools;

fn run(n: usize, a: Vec<usize>, b: Vec<usize>) -> isize {
    let a_vec: Vec<usize> = a.into_iter().sorted().rev().collect();
    let b_vec: Vec<usize> = b.into_iter().sorted().rev().chain(std::iter::once(0)).collect();

    let mut ans = 0;
    let mut b_diff = 0;

    for i in 0..n {
        if b_vec[i - b_diff] < a_vec[i] {
            if b_diff != 0 {
                return -1;
            }

            ans = a_vec[i] as isize;
            b_diff = 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![5, 2, 3, 7], vec![6, 2, 8], 3),
            TestCase(4, vec![5, 2, 3, 7], vec![6, 1, 8], -1),
            TestCase(8, vec![2, 28, 17, 39, 57, 56, 37, 32], vec![34, 27, 73, 28, 76, 61, 27], 37),
        ];

        for TestCase(n, a, b, expected) in tests {
            assert_eq!(run(n, a, b), expected);
        }
    }
}
