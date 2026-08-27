// https://atcoder.jp/contests/awc0036/tasks/awc0036_a

fn run(_n: usize, ms: Vec<(usize, Vec<usize>)>, _q: usize, vd: Vec<(usize, usize)>) -> (Vec<Vec<usize>>, usize) {
    let mut ng = 0;

    let mut s = Vec::new();

    for (_, vec) in ms {
        s.push(vec);
    }

    for (u, d) in vd {
        if s[u-1][d-1] > 0 {
            s[u-1][d-1] -= 1;
        } else {
            ng += 1;
        }
    }

    (s, ng)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, Vec<usize>)>, usize, Vec<(usize, usize)>, (Vec<Vec<usize>>, usize));

    #[test]
    fn awc0036_a() {
        let tests = [
            TestCase(2, vec![(2, vec![3, 1]), (1, vec![0])], 4, vec![(1, 1), (1, 2), (1, 2), (2, 1)], (vec![vec![2, 0], vec![0]], 2)),
            TestCase(3, vec![(3, vec![5, 0, 2]), (2, vec![1, 1]), (2, vec![3, 0])], 8, vec![(1, 2), (1, 1), (2, 1), (2, 2), (3, 2), (1, 3), (1, 3), (3, 1)], (vec![vec![4, 0, 0], vec![0, 0], vec![2, 0]], 2)),
            TestCase(4, vec![(3, vec![1000000000, 0, 5]), (4, vec![2, 3, 1, 0]), (1, vec![100]), (2, vec![10, 20])], 12, vec![(1, 1), (1, 2), (1, 2), (1, 3), (2, 4), (2, 4), (3, 1), (3, 1), (4, 1), (4, 2), (4, 2), (2, 3)], (vec![vec![999999999, 0, 4], vec![2, 3, 0, 0], vec![98], vec![9, 18]], 4)),
        ];

        for TestCase(n, ms, q, vd, expected) in tests {
            assert_eq!(run(n, ms, q, vd), expected);
        }
    }
}
