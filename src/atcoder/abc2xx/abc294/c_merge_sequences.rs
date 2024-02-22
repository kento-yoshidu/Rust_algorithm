// https://atcoder.jp/contests/abc294/tasks/abc294_c

use itertools::Itertools;

pub fn run(_n: usize, _m: usize, a: Vec<usize>, b: Vec<usize>) -> Vec<Vec<usize>> {
    let mut a_clone = a.clone();
    let mut b_clone = b.clone();

    a_clone.append(&mut b_clone);

    a_clone.sort();

    let a_ans: Vec<usize> = a_clone.iter()
        .positions(|num| {
            a.contains(num)
        })
        .map(|i| i + 1)
        .collect();

    let b_ans: Vec<usize> = a_clone.iter()
        .positions(|num| {
            b.contains(num)
        })
        .map(|i| i + 1)
        .collect();

    vec![a_ans, b_ans]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![vec![1, 3, 4, 7], vec![2, 5, 6]], run(4, 3, vec![3, 14, 15, 92], vec![6, 53, 58]));
        assert_eq!(vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]], run(4, 4, vec![1, 2, 3, 4,], vec![100, 200, 300, 400]));
        assert_eq!(vec![vec![1, 2, 5, 9, 11, 12, 15, 20], vec![3, 4, 6, 7, 8, 10, 13, 14, 16, 17, 18, 19]], run(8, 12, vec![3, 4, 10, 15, 17, 18, 22, 30], vec![5, 7, 11, 13, 14, 16, 19, 21, 23, 24, 27, 28]));
    }
}
