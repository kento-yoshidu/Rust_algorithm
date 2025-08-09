// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_bs

use itertools::Itertools;

fn run(_n: usize, a: Vec<usize>, b: Vec<usize>) -> usize {
    a.into_iter()
        .sorted()
        .zip(b.into_iter()
                .sorted()
                .rev()
                .collect::<Vec<usize>>())
        .map(|(a, b)| a * b)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>, usize);

    #[test]
    fn tessoku_a71() {
        let tests = [
            TestCase(3, vec![10, 20, 30], vec![35, 40, 33], 2090),
        ];

        for TestCase(n, a, b, expected) in tests {
            assert_eq!(run(n, a, b), expected);
        }
    }
}
