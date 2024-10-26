// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_n

use itertools::Itertools;

pub fn run(_n: usize, k: usize, a: Vec<usize>, b: Vec<usize>, c: Vec<usize>, d: Vec<usize>) -> &'static str {
    let mut p = Vec::new();
    let mut q = Vec::new();

    for i in a.iter() {
        for j in b.iter() {
            p.push(i + j);
        }
    }

    for i in c.iter() {
        for j in d.iter() {
            q.push(i + j);
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

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 50, vec![3, 9, 17], vec![4, 7, 9], vec![10, 20, 30], vec![1, 2, 3], "Yes"),
        ];

        for TestCase(n, k, a, b, c, d, expected) in tests {
            assert_eq!(run(n, k, a, b, c, d), expected);
        }
    }
}
