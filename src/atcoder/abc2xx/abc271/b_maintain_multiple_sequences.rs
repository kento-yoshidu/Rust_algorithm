// https://atcoder.jp/contests/abc271/tasks/abc271_b

pub fn run(n: usize, q: usize, l: Vec<Vec<usize>>, s: Vec<Vec<usize>>) -> Vec<usize> {
    s.iter()
        .map(|s_vec| {
            l[s_vec[0]-1][s_vec[1]]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![7, 5], run(2, 2, vec![vec![3, 1, 4, 7], vec![2, 5, 9]], vec![vec![1, 3], vec![2, 1]]));
        assert_eq!(vec![128, 1, 26535, 901], run(3, 4, vec![vec![4, 128, 741, 239, 901], vec![2, 1, 1], vec![3, 314, 159, 26535]], vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![1, 4]]));
    }
}
