// https://atcoder.jp/contests/joi2021yo1b/tasks/joi2021_yo1b_c

use itertools::Itertools;

fn run(_n: usize, a: Vec<usize>) -> (usize, usize) {
    let i = a.iter().position_max().unwrap();

    (a[..i].iter().sum(), a[i+1..].iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, (usize, usize));

    #[test]
    fn test() {
        let tests = [
            TestCase(5, vec![9, 3, 16, 8, 1], (12, 9)),
            TestCase(6, vec![121, 8, 5, 4, 1, 3], (0, 21)),
            TestCase(1, vec![2000], (0, 0)),
            TestCase(10, vec![9, 12, 30, 63, 55, 8, 10, 1, 27, 13], (51, 114)),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
