// https://atcoder.jp/contests/abc420/tasks/abc420_c

use std::cmp::min;

fn run(n: usize, _q: usize, a: Vec<usize>, b: Vec<usize>, cxv: Vec<(char, usize, usize)>) -> Vec<usize> {
    let mut a = a.clone();
    let mut b = b.clone();

    let mut min_vec = Vec::new();

    for i in 0..n {
        min_vec.push(min(a[i], b[i]));
    }

    let mut total: usize = min_vec.iter().sum();

    let mut ans = Vec::new();

    for (c, x, v) in cxv {
        total -= min(a[x-1], b[x-1]);

        match c {
            'A' => a[x-1] = v,
            'B' => b[x-1] = v,
            _ => unreachable!(),
        }

        total += min(a[x-1], b[x-1]);

        ans.push(total);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>, Vec<(char, usize, usize)>, Vec<usize>);

    #[test]
    fn abc420_c() {
        let tests = [
            TestCase(4, 3, vec![3, 1, 4, 1], vec![2, 7, 1, 8], vec![('A', 2, 3), ('B', 3, 3), ('A', 1, 7)], vec![7, 9, 9]),
            TestCase(1, 3, vec![1], vec![1000000000], vec![('A', 1, 1), ('A', 1, 1), ('A', 1, 1)], vec![1, 1, 1]),
            TestCase(5, 3, vec![100, 100, 100, 100, 100], vec![100, 100, 100, 100, 100], vec![('A', 4, 21), ('A', 2, 99), ('B', 4, 57)], vec![421, 420, 420]),
        ];

        for TestCase(n, q, a, b, cxv, expected) in tests {
            assert_eq!(run(n, q, a, b, cxv), expected);
        }
    }
}
