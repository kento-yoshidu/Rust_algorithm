// https://atcoder.jp/contests/abc188/tasks/abc188_c

use itertools::Itertools;

fn run(_n: usize, a: Vec<usize>) -> usize {
    let half = a.len()/2;

    let first = &a[0..half].iter().position_max().unwrap();
    let second = &a[half..].iter().position_max().unwrap();

    if &a[*first] > &a[half + *second] {
        half + *second+1
    } else {
        *first+1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc188_c() {
        let tests = [
            TestCase(2, vec![1, 4, 2, 5], 2),
            TestCase(2, vec![3, 1, 5, 4], 1),
            TestCase(4, vec![6, 13, 12, 5, 3, 7, 10, 11, 16, 9, 8, 15, 2, 1, 14, 4], 2),
            TestCase(1, vec![1000000000, 1], 2),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
