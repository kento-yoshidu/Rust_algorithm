// https://atcoder.jp/contests/abc378/tasks/abc378_b

fn run(_n: usize, qr: Vec<(usize, usize)>, _q: usize, qt: Vec<(usize, usize)>) -> Vec<usize> {
    let mut ans = Vec::new();

    for (t, d) in qt {
        let (q, r) = qr[t - 1];

        let res =
            if d % q <= r {
                (d / q) * q + r
            } else {
                (d / q + 1) * q + r
            };

        ans.push(res);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, usize, Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, vec![(7, 3), (4, 2)], 5, vec![(1, 1), (1, 3), (1, 4), (1, 15), (2, 7)], vec![3, 3, 10, 17, 10]),
        ];

        for TestCase(n, qr, q, qt, expected) in tests {
            assert_eq!(run(n, qr, q, qt), expected);
        }
    }
}
