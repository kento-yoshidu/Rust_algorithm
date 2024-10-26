// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_cm

use itertools::Itertools;

pub fn run(n: usize, k: usize, a: Vec<usize>) -> &'static str {
    let (l, r) = a.split_at(n/2);

    let mut p: Vec<usize> = Vec::new();
    let mut q: Vec<usize> = Vec::new();

    for i in 0..=l.len() {
        for combination in l.iter().combinations(i) {
            let sum = combination.iter().map(|&&x| x).sum();

            p.push(sum);
        }
    }

    for i in 0..=r.len() {
        for combination in r.iter().combinations(i) {
            let sum = combination.iter().map(|&&x| x).sum();

            q.push(sum);
        }
    }

    q.sort();

    for i in p.iter() {
        if q.binary_search(&(k - *i)).is_ok() {
            return "Yes";
        }
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, 30, vec![5, 1, 18, 7, 2, 9], "Yes"),
            TestCase(1, 648, vec![648], "Yes"),
            TestCase(1, 648, vec![1], "No"),
        ];

        for TestCase(n, k, a, expected) in tests {
            assert_eq!(run(n, k, a), expected);
        }
    }
}
