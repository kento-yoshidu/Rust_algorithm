// https://atcoder.jp/contests/awc0002/tasks/awc0002_d

use std::collections::VecDeque;
use itertools::Itertools;

fn run(_n: usize, _m: usize, c: Vec<usize>, r: Vec<usize>) -> usize {
    let c: Vec<usize> = c.into_iter().sorted().collect();
    let r: Vec<usize> = r.into_iter().sorted().collect();

    let mut c = VecDeque::from(c);
    let mut r = VecDeque::from(r);

    let mut ans = 0;

    let mut cur = c.pop_front().unwrap();

    while let Some(r) = r.pop_front() {
        if r >= cur {
            ans += 1;

            let Some(c) = c.pop_front() else {
                return ans;
            };

            cur = c;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>, usize);

    #[test]
    fn awc0002_d() {
        let tests = [
            TestCase(3, 3, vec![10, 20, 30], vec![15, 25, 35], 3),
            TestCase(5, 3, vec![100, 200, 300, 400, 500], vec![150, 250, 450], 3),
            TestCase(7, 10, vec![50, 120, 80, 200, 350, 500, 1000], vec![40, 60, 100, 100, 150, 200, 300, 400, 600, 1500], 7),
        ];

        for TestCase(n, m, c, r, expected) in tests {
            assert_eq!(run(n, m, c, r), expected);
        }
    }
}
