// https://atcoder.jp/contests/abc291/tasks/abc291_b

use itertools::Itertools;

fn run(n: usize, x: Vec<usize>) -> f64 {
    x.iter()
        .sorted()
        .enumerate()
        .filter(|(i, _)| {
            !(i < &n || x.len() - n <= *i)
        })
        .map(|(_, num)| *num as f64)
        .sum::<f64>() / (x.len() - n*2) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, f64);

    #[test]
    fn abc291_b() {
        let tests = [
            TestCase(1, vec![10, 100, 20, 50, 30], 33.333333333333336),
            TestCase(2, vec![3, 3, 3, 4, 5, 6, 7, 8, 99, 100], 5.5),
        ];

        for TestCase(n, x, expected) in tests {
            assert_eq!(run(n, x), expected);
        }
    }
}
