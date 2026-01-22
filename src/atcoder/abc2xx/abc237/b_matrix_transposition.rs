// https://atcoder.jp/contests/abc237/tasks/abc237_b

fn run(h: usize, w: usize, a: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut ans = vec![vec![0; h]; w];

    for i in 0..w {
        for j in 0..h {
            ans[i][j] = a[j][i];
        }
    }

    ans
}

fn run2(h: usize, w: usize, a: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    (0..w).map(|i| {
        (0..h).map(|j| {
            a[j][i]
        })
        .collect()
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<Vec<usize>>, Vec<Vec<usize>>);

    #[test]
    fn abc237_b() {
        let tests = [
            TestCase(4, 3, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![10, 11, 12]], vec![vec![1, 4, 7, 10], vec![2, 5, 8, 11], vec![3, 6, 9, 12]]),
            TestCase(2, 2, vec![vec![1000000000, 1000000000], vec![1000000000, 1000000000]], vec![vec![1000000000, 1000000000], vec![1000000000, 1000000000]]),
        ];

        for TestCase(h, w, a, expected) in tests {
            assert_eq!(run(h, w, &a), expected);
            assert_eq!(run2(h, w, &a), expected);
        }
    }
}
