// https://atcoer.jp/contests/abc246/tasks/abc246_c

use std::cmp::min;

fn run(n: usize, k: usize, x: usize, a: Vec<usize>) -> usize {
    let mut ans: usize = a.iter().sum();
    let mut rest = k;

    // クーポンを何回使えるか
    let count = a.iter()
        .map(|n| n / x)
        .sum::<usize>();

    let mut arr = a.iter()
        .map(|n| n % x)
        .collect::<Vec<usize>>();

    ans -= min(rest, count) * x;
    rest -= min(rest, count);

    arr.sort();
    arr.reverse();

    for i in 0..min(rest, n) {
        ans -= arr[i];
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, usize);

    #[test]
    fn abc246_c() {
        let tests = [
            TestCase(5, 4, 7, vec![8, 3, 10, 5, 13], 12),
            TestCase(5, 100, 7, vec![8, 3, 10, 5, 13], 0),
            TestCase(20, 815, 60, vec![2066, 3193, 2325, 4030, 3725, 1669, 1969, 763, 1653, 159, 5311, 5341, 4671, 2374, 4513, 285, 810, 742, 2981, 202], 112),
        ];

        for TestCase(n, k, x, a, expected) in tests {
            assert_eq!(run(n, k, x, a), expected);
        }
    }
}
