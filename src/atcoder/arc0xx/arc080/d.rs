// https://atcoder.jp/contests/abc069/tasks/arc080_b

fn run(h: usize, w: usize, _n: usize, a: Vec<usize>) -> Vec<Vec<usize>> {
    let mut ans = vec![vec![0; w]; h];

    let mut color = 1;
    let mut rest = a[0];

    for i in 0..h {
        if i % 2 == 0 {
            for j in 0..w {
                ans[i][j] = color;
                rest -= 1;

                if rest == 0 {
                    color += 1;
                    if color <= a.len() {
                        rest = a[color - 1];
                    }
                }
            }
        } else {
            for j in (0..w).rev() {
                ans[i][j] = color;
                rest -= 1;

                if rest == 0 {
                    color += 1;
                    if color <= a.len() {
                        rest = a[color - 1];
                    }
                }
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, Vec<Vec<usize>>);

    #[test]
    fn abc069_d() {
        let tests = [
            TestCase(2, 2, 3, vec![2, 1, 1], vec![vec![1, 1], vec![3, 2]]),
            TestCase(3, 5, 5, vec![1, 2, 3, 4, 5], vec![vec![1, 2, 2, 3, 3], vec![4, 4, 4, 4, 3], vec![5, 5, 5, 5, 5]]),
            TestCase(1, 1, 1, vec![1], vec![vec![1]]),
        ];

        for TestCase(h, w, n, a, expected) in tests {
            assert_eq!(run(h, w, n, a), expected);
        }
    }
}
