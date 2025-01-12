// https://atcoder.jp/contests/abc388/tasks/abc388_c

use library::lib::upper_bound::upper_bound;

fn run(_n: usize, a: Vec<usize>) -> usize {
    a.iter()
        .map(|x| {
            upper_bound(&a, x/2)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, vec![2, 3, 4, 4, 7, 10], 8),
            TestCase(3, vec![387, 388, 389], 0),
            TestCase(32, vec![1, 2, 4, 5, 8, 10, 12, 16, 19, 25, 33, 40, 50, 64, 87, 101, 149, 175, 202, 211, 278, 314, 355, 405, 412, 420, 442, 481, 512, 582, 600, 641], 388),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
