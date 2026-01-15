// https://atcoder.jp/contests/abc271/tasks/abc271_b

fn run(_n: usize, _q: usize, l: Vec<Vec<usize>>, s: Vec<Vec<usize>>) -> Vec<usize> {
    s.into_iter()
        .map(|v| {
            l[v[0]-1][v[1]]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<Vec<usize>>, Vec<Vec<usize>>, Vec<usize>);

    #[test]
    fn abc271_b() {
        let tests = [
            TestCase(2, 2, vec![vec![3, 1, 4, 7], vec![2, 5, 9]], vec![vec![1, 3], vec![2, 1]], vec![7, 5]),
            TestCase(3, 4, vec![vec![4, 128, 741, 239, 901], vec![2, 1, 1], vec![3, 314, 159, 26535]], vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![1, 4]], vec![128, 1, 26535, 901]),
        ];

        for TestCase(n, q, l, s, expected) in tests {
            assert_eq!(run(n, q, l, s), expected);
        }
    }
}
