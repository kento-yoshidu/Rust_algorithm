// https://atcoder.jp/contests/abc462/tasks/abc462_c

use itertools::Itertools;

fn run(_n: usize, xy: Vec<(usize, usize)>) -> usize {
    let xy: Vec<(usize, usize)> = xy.into_iter().sorted_by(|a, b| a.0.cmp(&(b.0))).collect();

    let mut min_y = std::usize::MAX;

    let mut ans = 0;

    for (_, y) in xy {
        if y < min_y {
            min_y = y;
            ans += 1;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, usize);

    #[test]
    fn abc462_c() {
        let tests = [
            TestCase(3, vec![(2, 1), (1, 3), (3, 2)], 2),
            TestCase(5, vec![(1, 1), (4, 2), (2, 3), (5, 5), (3, 4)], 1),
            TestCase(7, vec![(3, 4), (6, 1), (5, 5), (2, 7), (7, 2), (1, 3), (4, 6)], 2),
        ];

        for TestCase(n, xy, expected) in tests {
            assert_eq!(run(n, xy), expected);
        }
    }
}
