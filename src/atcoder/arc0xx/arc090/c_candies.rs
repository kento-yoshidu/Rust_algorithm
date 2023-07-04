// https://atcoder.jp/contests/abc087/tasks/arc090_a

#[allow(dead_code)]
pub fn run(n: usize, vec: Vec<Vec<usize>>) -> usize {
    let mut ans = 0;

    // a番目で下の列に移動する
    for a in 0..n {
        let mut total = 0;

        for i in 0..=a {
            total += vec[0][i];
        }

        for j in a..n {
            total += vec[1][j];
        }

        ans = ans.max(total);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(14, run(5, vec![vec![3, 2, 2, 4, 1], vec![1, 2, 2, 2, 1]]));
        assert_eq!(5, run(4, vec![vec![1, 1, 1, 1], vec![1, 1, 1, 1]]));
        assert_eq!(29, run(7, vec![vec![3, 3, 4, 5, 4, 5, 3], vec![5, 3, 4, 4, 2, 3, 2]]));
        assert_eq!(5, run(1, vec![vec![2], vec![3]]));
    }
}
