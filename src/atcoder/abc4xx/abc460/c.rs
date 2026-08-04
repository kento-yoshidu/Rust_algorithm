// https://atcoder.jp/contests/abc460/tasks/abc460_c

use std::collections::VecDeque;

use itertools::Itertools;

fn run(_n: usize, _m: usize, a: Vec<usize>, b: Vec<usize>) -> usize {
    let a: Vec<usize> = a.into_iter().sorted().collect();
    let b: Vec<usize> = b.into_iter().sorted().collect();

    let mut a = VecDeque::from(a);
    let mut b = VecDeque::from(b);

    let mut ans = 0;

    loop {
        let Some(syari) = a.pop_front() else {
            return ans;
        };

        if b.is_empty() {
            return ans;
        }

        if b.front().copied() > Some(syari * 2) {
            continue;
        } else {
            ans += 1;
            b.pop_front();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>, usize);

    #[test]
    fn abc460_c() {
        let tests = [
            TestCase(4, 5, vec![4, 2, 1, 8], vec![14, 9, 3, 2, 9], 3),
            TestCase(3, 3, vec![5, 5, 3], vec![11, 1000, 1000], 0),
            TestCase(8, 7, vec![2, 3, 4, 4, 4, 3, 2, 3], vec![8, 5, 5, 9, 9, 7, 1], 5),
        ];

        for TestCase(n, m, a, b, expected) in tests {
            assert_eq!(run(n, m, a, b), expected);
        }
    }
}
