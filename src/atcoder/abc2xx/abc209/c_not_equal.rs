// https://atcoder.jp/contests/abc209/tasks/abc209_c

use itertools::Itertools;

fn run(_n: usize, c: Vec<usize>) -> usize {
    let m = 1_000_000_007;

    c.into_iter()
        .sorted()
        .enumerate()
        .fold(1, |state, (i, num)| {
            (state * (num - i)) % m
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc209_c() {
        let tests = [
            TestCase(2, vec![1, 3], 2),
            TestCase(4, vec![3, 3, 4, 4], 12),
            TestCase(2, vec![1, 1], 0),
            TestCase(10, vec![999999917, 999999914, 999999923, 999999985, 999999907, 999999965, 999999914, 999999908, 999999951, 999999979], 405924645),
        ];

        for TestCase(n, c, expected) in tests {
            assert_eq!(run(n, c), expected);
        }
    }
}

// https://note.com/momomo0214/n/n85a741e01054
// https://qiita.com/sano192/items/faf544347362c3f89630
