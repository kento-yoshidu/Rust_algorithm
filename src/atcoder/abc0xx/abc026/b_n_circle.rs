// https://atcoder.jp/contests/abc026/tasks/abc026_b

use std::f64::consts::PI;
use itertools::Itertools;

fn run(_n: i32, r: Vec<i32>) -> f64 {
    let vec: Vec<i32> = r.into_iter().sorted().rev().collect();

    vec.iter()
        .enumerate()
        .map(|(i, num)| {
            if i % 2 == 0 {
                num * num
            } else {
                -(num * num)
            }
        })
        .sum::<i32>() as f64 * PI
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(i32, Vec<i32>, f64);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![1, 2, 3], 18.84955592153876),
            TestCase(6, vec![15, 2, 3, 7, 6, 9], 508.93800988154646),
        ];

        for TestCase(n, r, expected) in tests {
            assert_eq!(run(n, r), expected);
        }
    }
}
