// https://atcoder.jp/contests/abc143/tasks/abc143_d

use itertools::Itertools;

use library::lib::binary_search::upper_bound::*;

fn run(n: usize, l: Vec<usize>) -> usize {
    let vec: Vec<usize> = l.into_iter().sorted().collect();

    let mut ans = 0;

    for i in 0..n {
        for j in i+1..n {
            let res = upper_bound(&vec, vec[i] + vec[j] - 1);

            if res > j + 1 {
                ans += res - j - 1;
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc143_d() {
        let tests = [
            TestCase(4, vec![3, 4, 2, 1], 1),
            TestCase(3,vec![1, 1000, 1], 0),
            TestCase(7, vec![218, 786, 704, 233, 645, 728, 389], 23),
        ];

        for TestCase(n, l, expected) in tests {
            assert_eq!(run(n, l), expected);
        }
    }
}
