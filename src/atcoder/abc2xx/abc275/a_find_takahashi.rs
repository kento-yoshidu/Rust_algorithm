// https://atcoder.jp/contests/abc275/tasks/abc275_a

use itertools::Itertools;

fn run(_n: usize, h: Vec<usize>) -> usize {
    let max = h.iter().max().unwrap();

    h.iter().position(|num| num == max).unwrap() + 1
}

fn run2(_n: usize, h: Vec<usize>) -> usize {
    h.into_iter()
        .enumerate()
        .sorted_by(|a, b| b.1.cmp(&a.1))
        .nth(0)
        .map(|(i, _)| i+1)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![50, 80, 70], 2),
            TestCase(1, vec![1000000000], 1),
            TestCase(10, vec![22, 75, 26, 45, 72, 81, 47, 29, 97, 2], 9),
        ];

        for TestCase(n, h, expected) in tests {
            assert_eq!(run(n, h), expected);
        }
    }
}
