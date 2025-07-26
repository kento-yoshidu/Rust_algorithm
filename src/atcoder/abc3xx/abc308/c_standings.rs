// https://atcoder.jp/contests/abc308/tasks/abc308_c

use std::cmp::Ordering;

fn run(_n: usize, ab: Vec<(usize, usize)>) -> Vec<usize> {
    let mut ans = ab.into_iter().enumerate().collect::<Vec<_>>();

    ans.sort_by(|&(i, (a1, b1)), &(j, (a2, b2))| {
        let lhs = a1 * (a2 + b2);
        let rhs = a2 * (a1 + b1);

        match rhs.cmp(&lhs) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => i.cmp(&j),
        }
    });

    ans.iter().map(|(i, _)| i + 1).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn abc308_c() {
        let tests = [
            TestCase(3, vec![(1, 3), (3, 1), (2, 2)], vec![2, 3, 1]),
            TestCase(2, vec![(1, 3), (2, 6)], vec![1, 2]),
            TestCase(4, vec![(999999999, 1000000000), (333333333, 999999999), (1000000000, 999999997), (999999998, 1000000000)], vec![3, 1, 4, 2]),
        ];

        for TestCase(n, ab, expected) in tests {
            assert_eq!(run(n, ab), expected);
        }
    }
}
