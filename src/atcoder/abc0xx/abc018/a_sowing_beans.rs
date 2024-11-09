// https://atcoder.jp/contests/abc018/tasks/abc018_1

use itertools::Itertools;

fn run(a: usize, b: usize, c: usize) -> Vec<usize> {
    let vec = vec![a, b, c];
    let sorted_vec: Vec<usize> = vec.clone().into_iter().sorted().rev().collect();

    vec.into_iter()
        .map(|i| {
            sorted_vec.iter().position(|x| *x == i).unwrap() + 1
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(12, 18, 11, vec![2, 1, 3]),
            TestCase(10, 20, 30, vec![3, 2, 1]),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
