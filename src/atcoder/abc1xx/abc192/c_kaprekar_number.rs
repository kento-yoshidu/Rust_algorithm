// https://atcoder.jp/contests/abc192/tasks/abc192_c

use itertools::Itertools;

fn f1(num: usize) -> usize {
    let s: String = num.to_string().chars().sorted().collect();
    s.parse::<usize>().unwrap()
}

fn f2(num: usize) -> usize {
    let s: String = num.to_string().chars().sorted().rev().collect();
    s.parse::<usize>().unwrap()
}

pub fn run(n: usize, k: usize) -> usize {
    let mut x = n;

    for _ in 0..k {
        x = f2(x) - f1(x);
    }

    x
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc192_c() {
        let tests = [
            TestCase(314, 2, 693),
            TestCase(1000000000, 100, 0),
            TestCase(6174, 100000, 6174),
            TestCase(335279264, 100000, 865296432),
        ];

        for TestCase(n, k, expected) in tests {
            assert_eq!(run(n, k), expected);
        }
    }
}
