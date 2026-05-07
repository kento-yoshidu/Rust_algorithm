// https://atcoder.jp/contests/abc451/tasks/abc451_b

fn run(_n: usize, m: usize, ab: Vec<(usize, usize)>) -> Vec<isize> {
    let mut cur = vec![0; m + 1];
    let mut new = vec![0; m + 1];

    for (a, b) in ab {
        cur[a] += 1;
        new[b] += 1;
    }

    cur.into_iter()
        .zip(new)
        .skip(1)
        .map(|(a, b)| b - a)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, Vec<isize>);

    #[test]
    fn abc451_b() {
        let tests = [
            TestCase(5, 4, vec![(1, 2), (2, 1), (3, 1), (2, 2), (2, 4)], vec![1, -1, -1, 1]),
            TestCase(10, 5, vec![(3, 2), (3, 4), (1, 2), (2, 2), (4, 4), (3, 1), (3, 4), (4, 2), (3, 3), (3, 2)], vec![0, 4, -5, 1, 0]),
        ];

        for TestCase(n, m, ab, expected) in tests {
            assert_eq!(run(n, m, ab), expected);
        }
    }
}
