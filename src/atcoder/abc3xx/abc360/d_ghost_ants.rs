// https://atcoder.jp/contests/abc360/tasks/abc360_d

use library::lib::binary_search::{lower_bound::*, upper_bound::*};

fn run(_n: usize, t: usize, s: &str, x: Vec<isize>) -> usize {
    let mut to_left = Vec::new();
    let mut to_right = Vec::new();

    for (i, c) in s.chars().enumerate() {
        match c {
            '1' => to_right.push(&x[i]),
            '0' => to_left.push(&x[i]),
            _ => unreachable!(),
        }
    }

    to_left.sort();
    to_right.sort();

    to_right.into_iter()
        .map(|l| {
            let r = *l + (t * 2) as isize;
            upper_bound(&to_left, &r) - lower_bound(&to_left, &l)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str, Vec<isize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, 3, "101010", vec![-5, -1, 0, 1, 2, 4], 5),
            TestCase(13, 656320850, "0100110011101", vec![-900549713, -713494784, -713078652, -687818593, -517374932, -498415009, -472742091, -390030458, -379340552, -237481538, -44636942, 352721061, 695864366], 14),
        ];

        for TestCase(n, t, s, x, expected) in tests {
            assert_eq!(run(n, t, s, x), expected);
        }
    }
}
