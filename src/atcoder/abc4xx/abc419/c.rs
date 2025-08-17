// https://atcoder.jp/contests/abc419/tasks/abc419_c

use std::collections::BTreeSet;
use std::cmp::max;

fn run(_n: usize, rc: Vec<(usize, usize)>) -> usize {
    let mut x = BTreeSet::new();
    let mut y = BTreeSet::new();

    for (r, c) in rc {
        x.insert(r);
        y.insert(c);
    }

    let x_min = x.iter().next().unwrap();
    let x_max = x.iter().next_back().unwrap();
    let y_min = y.iter().next().unwrap();
    let y_max = y.iter().next_back().unwrap();

    (max(x_max - x_min, y_max - y_min) + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, usize);

    #[test]
    fn abc419_c() {
        let tests = [
            TestCase(3, vec![(2, 3), (5, 1), (8, 1)], 3),
            TestCase(5, vec![(6, 7), (6, 7), (6, 7), (6, 7), (6, 7)], 0),
            TestCase(6, vec![(91, 999999986), (53, 999999997), (32, 999999932), (14, 999999909), (49, 999999985), (28, 999999926)], 44),
        ];

        for TestCase(n, rc, expected) in tests {
            assert_eq!(run(n, rc), expected);
        }
    }
}
