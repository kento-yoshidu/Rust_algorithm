// https://atcoder.jp/contests/abc358/tasks/abc358_d

use itertools::Itertools;

pub fn run(n: usize, m: usize, a: Vec<usize>, b: Vec<usize>) -> isize {
    let mut ans = 0;

    let vec_a: Vec<usize> = a.into_iter().sorted().collect();
    let vec_b: Vec<usize> = b.into_iter().sorted().collect();

    let mut a_pos = 0;

    for i in 0..m {
        loop {
            if a_pos == n {
                return -1;
            }

            if vec_a[a_pos] >= vec_b[i] {
                ans += vec_a[a_pos] as isize;
                a_pos += 1;
                break;
            }

            a_pos += 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 2, vec![3, 4, 5, 4], vec![1, 4], 7),
            TestCase(3, 3, vec![1, 1, 1], vec![1 ,1, 1], 3),
            TestCase(3, 3, vec![1, 1, 1], vec![1000000000, 1000000000, 1000000000], -1),
            TestCase(7, 3, vec![2, 6, 8, 9, 5, 1, 11], vec![3, 5, 7], 19),
        ];

        for TestCase(n, m, a, b, expected) in tests {
            assert_eq!(run(n, m, a, b), expected);
        }
    }
}
