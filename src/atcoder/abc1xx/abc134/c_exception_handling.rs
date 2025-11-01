// https://atcoder.jp/contests/abc134/tasks/abc134_c

use itertools::Itertools;

fn run(_n: usize, p: Vec<usize>) -> Vec<usize> {
    let vec: Vec<usize> = p.clone().into_iter().sorted().rev().collect();

    let max = vec[0];
    let next = vec[1];

    p.into_iter()
        .map(|num| {
            if num == max {
                next
            } else {
                max
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>);

    #[test]
    fn abc134_c() {
        let tests = [
            TestCase(3, vec![1, 4, 3], vec![4, 3, 4]),
            TestCase(3, vec![1, 1, 1, 1, 5], vec![5, 5, 5, 5, 1]),
            TestCase(2, vec![5, 5], vec![5, 5]),
        ];

        for TestCase(n, p, expected) in tests {
            assert_eq!(run(n, p), expected);
        }
    }
}
