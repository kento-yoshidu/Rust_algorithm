// https://atcoder.jp/contests/abc365/tasks/abc365_c

use std::cmp::min;

fn check(a: &Vec<usize>, x: usize, m: usize) -> bool {
    let mut total = 0;

    for n in a.iter() {
        total += min(n, &x);
    }

    total <= m
}

pub fn run(_n: usize, m: usize, a: Vec<usize>) -> String {
    let sum: usize = a.iter().sum();

    if sum <= m {
        return "infinite".to_string();
    }

    let mut l = 0;
    let mut r = std::usize::MAX;

    while l+1 < r {
        let tmp = (l+r)/2;

        if check(&a, tmp, m) == true {
            l = tmp;
        } else {
            r = tmp;
        }
    }

    l.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 8, vec![1, 3, 2, 4], "2"),
            TestCase(3, 20, vec![5, 3, 2], "infinite"),
            TestCase(10, 23, vec![2, 5, 6, 5, 2, 1, 7, 9, 7, 2], "2"),
        ];

        for TestCase(n, m, a, expected) in tests {
            assert_eq!(run(n, m, a), expected);
        }
    }
}