// https://atcoder.jp/contests/abc293/tasks/abc293_b

use itertools::Itertools;

pub fn run(n: usize, a: Vec<usize>) -> (usize, Vec<usize>) {
    let mut vec = vec![false; n];

    for i in 0..n {
        if vec[i] == false {
            vec[a[i]-1] = true;
        }
    }

    let ans: Vec<usize> = vec.iter()
        .positions(|i| *i == false)
        .map(|i| i+1)
        .collect();

    (ans.len(), ans )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!((2, vec![2, 4]), run(5, vec![3, 1, 4, 5, 4]));
        assert_eq!((10, vec![1, 2, 5, 6, 8, 11, 14, 17, 18, 20]), run(20, vec![9, 7, 19, 7, 10, 4, 13, 9, 4, 8, 10, 15, 16, 3, 18, 19, 12, 13, 2, 12]));
    }
}
