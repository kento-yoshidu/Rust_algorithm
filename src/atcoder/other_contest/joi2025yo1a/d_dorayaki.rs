// https://atcoder.jp/contests/joi2025yo1a/tasks/joi2025_yo1a_d

use std::cmp::max;

fn run(_n: usize, _m: usize, a: Vec<usize>, b: Vec<usize>) -> usize {
    let mut ans = 0;

    for x in a.iter() {
        for y in b.iter() {
            ans += (x+y) * max(x, y);
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 2, vec![1, 2], vec![2, 5], 79),
        ];

        for TestCase(n, m, a, b, expected) in tests {
            assert_eq!(run(n, m, a, b), expected);
        }
    }
}
