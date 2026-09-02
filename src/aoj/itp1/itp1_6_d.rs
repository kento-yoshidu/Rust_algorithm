// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/6/ITP1_6_D

fn run(n: usize, m: usize, a: Vec<Vec<usize>>, b: Vec<usize>) -> Vec<usize> {
    (0..n).map(|i| {
        (0..m).map(|j| a[i][j] * b[j]).sum()
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<Vec<usize>>, Vec<usize>, Vec<usize>);

    #[test]
    fn itp1_6_a() {
        let tests = [
            TestCase(3, 4, vec![vec![1, 2, 0, 1], vec![0, 3, 0, 1], vec![4, 1, 1, 0]], vec![1, 2, 3, 0], vec![5, 6, 9]),
        ];

        for TestCase(n, m, a, b, expected) in tests {
            assert_eq!(run(n, m, a, b), expected);
        }
    }
}
