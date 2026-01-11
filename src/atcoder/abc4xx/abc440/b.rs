// https://atcoder.jp/contests/abc440/tasks/abc440_b

use itertools::Itertools;

fn run(_n: usize, t: Vec<usize>) -> Vec<usize> {
    t.into_iter()
        .enumerate()
        .sorted_by(|a, b| a.1.cmp(&(b.1)))
        .take(3)
        .map(|(i, _)| i+1)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>);

    #[test]
    fn abc440_b() {
        let tests = [
            TestCase(4, vec![100, 110, 105, 95], vec![4, 1, 3]),
            TestCase(8, vec![72, 74, 69, 70, 73, 75, 71, 77], vec![3, 4, 7]),
        ];

        for TestCase(n, t, expected) in tests {
            assert_eq!(run(n, t), expected);
        }
    }
}
