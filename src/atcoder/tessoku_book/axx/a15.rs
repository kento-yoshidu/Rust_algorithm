// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_o

use itertools::Itertools;

fn run(_n: usize, a: Vec<usize>) -> Vec<usize> {
    let vec: Vec<&usize> = a.iter().sorted().dedup().collect();

    let mut ans = Vec::new();

    for num in a.iter() {
        ans.push(vec.binary_search(&num).unwrap() + 1);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>);

    #[test]
    fn tessoku_a15() {
        let tests = [
            TestCase(5, vec![46, 80, 11, 77, 46], vec![2, 4, 1, 3, 2]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
