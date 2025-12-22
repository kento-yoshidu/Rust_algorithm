// https://atcoder.jp/contests/abc213/tasks/abc213_b

use itertools::Itertools;

fn run(n: usize, a: Vec<usize>) -> usize {
    let sorted: Vec<&usize> = a.iter().sorted().collect();

    let num = sorted[n-2];

    a.iter()
        .position(|i| {
            i == num
        })
        .unwrap() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc213_b() {
        let tests = [
            TestCase(6, vec![1, 123, 12345, 12, 1234, 123456], 3),
            TestCase(5, vec![3, 1, 4, 15, 9], 5),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
