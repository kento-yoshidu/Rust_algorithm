// https://atcoder.jp/contests/abc237/tasks/abc237_b

pub fn run(h: usize, w: usize, a: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut ans = vec![vec![0; h]; w];

    for i in 0..w {
        for j in 0..h {
            ans[i][j] = a[j][i];
        }
    }

    ans
}

pub fn run2(h: usize, w: usize, a: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
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

    #[test]
    fn test() {
        assert_eq!(vec![vec![1, 4, 7, 10], vec![2, 5, 8, 11], vec![3, 6, 9, 12]], run(4, 3, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![10, 11, 12]]));
        assert_eq!(vec![vec![1000000000, 1000000000], vec![1000000000, 1000000000]], run(2, 2, vec![vec![1000000000, 1000000000], vec![1000000000, 1000000000]]));
    }

    #[test]
    fn test2() {
        assert_eq!(vec![vec![1, 4, 7, 10], vec![2, 5, 8, 11], vec![3, 6, 9, 12]], run2(4, 3, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![10, 11, 12]]));
        assert_eq!(vec![vec![1000000000, 1000000000], vec![1000000000, 1000000000]], run2(2, 2, vec![vec![1000000000, 1000000000], vec![1000000000, 1000000000]]));
    }
}
