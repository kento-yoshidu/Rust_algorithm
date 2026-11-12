// https://atcoder.jp/contests/awc0034/tasks/awc0034_a

use std::cmp::min;

fn run(n: usize, _m: usize, c: Vec<usize>, t: Vec<usize>) -> usize {
    let mut count = vec![0; n];

    for t in t {
        count[t-1] += 1;
    }

    (1..=n)
        .map(|i| {
            min(c[i-1], count[i-1])
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>, usize);

    #[test]
    fn awc0034_a() {
        let tests = [
            TestCase(3, 5, vec![2, 1, 3], vec![1, 2, 1, 1, 2], 3),
            TestCase(4, 10, vec![3, 2, 1, 4], vec![1, 2, 3, 4, 1, 2, 3, 1, 4, 4], 9),
            TestCase(5, 15, vec![1000000000, 1, 2, 3, 1], vec![1, 1, 1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 1], 11),
        ];

        for TestCase(n, m, c, t, expected) in tests {
            assert_eq!(run(n, m, c, t), expected);
        }
    }
}
