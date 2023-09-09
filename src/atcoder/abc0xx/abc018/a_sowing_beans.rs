// https://atcoder.jp/contests/abc018/tasks/abc018_1

use itertools::Itertools;

pub fn run(a: usize, b: usize, c: usize) -> Vec<usize> {
    let vec = vec![a, b, c];
    let sorted_vec: Vec<usize> = vec.clone().into_iter().sorted().rev().collect();

    vec.iter().map(|i| {
        sorted_vec.iter().position(|x| x == i).unwrap() + 1
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![2, 1, 3], run(12, 18, 11));
        assert_eq!(vec![3, 2, 1], run(10, 20, 30));
    }
}
