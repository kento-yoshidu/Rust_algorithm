// https://atcoder.jp/contests/abc335/tasks/abc335_b

pub fn run(n: usize) -> Vec<Vec<usize>> {
    let mut ans = Vec::new();

    for i in 0..=n {
        for j in 0..=n {
            for k in 0..=n {
                if i + j + k > n {
                    continue;
                }

                ans.push(vec![i, j, k]);
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![vec![0, 0, 0], vec![0, 0, 1], vec![0, 0, 2], vec![0, 0, 3], vec![0, 1, 0], vec![0, 1, 1], vec![0, 1, 2], vec![0, 2, 0], vec![0, 2, 1], vec![0, 3, 0],
                    vec![1, 0, 0], vec![1, 0, 1], vec![1, 0, 2], vec![1, 1, 0], vec![1, 1, 1], vec![1, 2, 0],
                    vec![2, 0, 0], vec![2, 0, 1], vec![2, 1, 0],
                    vec![3, 0, 0]], run(3));
        assert_eq!(vec![vec![0, 0, 0], vec![0, 0, 1], vec![0, 0, 2], vec![0, 0, 3], vec![0, 0, 4], vec![0, 1, 0], vec![0, 1, 1], vec![0, 1, 2], vec![0, 1, 3], vec![0, 2, 0], vec![0, 2, 1], vec![0, 2, 2], vec![0, 3, 0], vec![0, 3, 1], vec![0, 4, 0],
                    vec![1, 0, 0], vec![1, 0, 1], vec![1, 0, 2], vec![1, 0, 3], vec![1, 1, 0], vec![1, 1, 1], vec![1, 1, 2], vec![1, 2, 0], vec![1, 2, 1], vec![1, 3, 0],
                    vec![2, 0, 0], vec![2, 0, 1], vec![2, 0, 2], vec![2, 1, 0], vec![2, 1, 1], vec![2, 2, 0],
                    vec![3, 0, 0], vec![3, 0, 1], vec![3, 1, 0],
                    vec![4, 0, 0]], run(4));
    }
}
