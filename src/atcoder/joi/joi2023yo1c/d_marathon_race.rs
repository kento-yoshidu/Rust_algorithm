// https://atcoder.jp/contests/joi2023yo1c/tasks/joi2023_yo1c_d

use itertools::Itertools;

fn run(_n: usize, a: Vec<usize>) -> Vec<usize> {
    let arr: Vec<&usize> = a.iter().sorted().collect();

    a.iter()
        .map(|n| {
            arr.iter().position(|x| **x == *n).unwrap() + 1
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![44, 42, 69], vec![2, 1, 3]),
            TestCase(4, vec![40, 60, 40, 60], vec![1, 3, 1, 3]),
            TestCase(10, vec![766, 152, 595, 926, 663, 509, 368, 595, 175, 622], vec![9, 1, 5, 10, 8, 4, 3, 5, 2, 7]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
