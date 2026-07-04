// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/7/ITP1_7_D

fn run(n: usize, m: usize, l: usize, a: Vec<Vec<usize>>, b: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut ans = Vec::new();

    for i in 0..n {
        let mut row = Vec::new();

        for j in 0..l {
            let mut sum = 0;

            for k in 0..m {
                sum += a[i][k] * b[k][j];
            }

            row.push(sum);
        }

        ans.push(row);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<Vec<usize>>, Vec<Vec<usize>>, Vec<Vec<usize>>);

    #[test]
    fn itp1_7_d() {
        let tests = [
            TestCase(
                3,
                2,
                3,
                vec![vec![1, 2], vec![0, 3], vec![4, 5]],
                vec![vec![1, 2, 1], vec![0, 3, 2]],
                vec![vec![1, 8, 5], vec![0, 9, 6], vec![4, 23, 14]],
            ),
        ];

        for TestCase(n, m, l, a, b, expected) in tests {
            assert_eq!(run(n, m, l, a, b), expected);
        }
    }
}
