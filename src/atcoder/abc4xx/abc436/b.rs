// https://atcoder.jp/contests/abc436/tasks/abc436_b

fn run(n: usize) -> Vec<Vec<usize>> {
    let mut ans = vec![vec![0; n]; n];

    ans[0][(n - 1) / 2] = 1;

    let mut rc = (0, (n - 1) / 2);
    let mut k = 1;

    for _ in 0..n * n - 1 {
        let x = (rc.0 + n - 1) % n;
        let y = (rc.1 + 1) % n;

        k += 1;

        if ans[x][y] == 0 {
            ans[x][y] = k;
            rc = (x, y);
        } else {
            let nx = (rc.0 + 1) % n;
            ans[nx][rc.1] = k;
            rc = (nx, rc.1);
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<Vec<usize>>);

    #[test]
    fn abc436_b() {
        let tests = [
            TestCase(3, vec![vec![8, 1, 6], vec![3, 5, 7], vec![4, 9, 2]]),
            TestCase(5, vec![vec![17, 24, 1, 8, 15], vec![23, 5, 7, 14, 16], vec![4, 6, 13, 20, 22], vec![10, 12, 19, 21, 3], vec![11, 18, 25, 2, 9]]),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
