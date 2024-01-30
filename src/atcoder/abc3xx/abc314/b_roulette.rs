// https://atcoder.jp/contests/abc315/tasks/abc315_b

use itertools::Itertools;

pub fn run(_n: usize, a: Vec<Vec<usize>>, x: usize) -> (usize, Vec<usize>) {
    // xを含むvecを、要素数が小さい順に抜き出し
    let new_vec = a
        .iter()
        .enumerate()
        .filter(|(_, vec)| vec.contains(&x))
        .sorted_by_key(|(_, vec)| vec.len())
        .collect_vec();

    if new_vec.len() == 0 {
        return (0, vec![0])
    }

    // 要素数の最小数
    let min = new_vec[0].1.len();

    let ans = new_vec
        .iter()
        .filter(|(_, vec)| vec.len() == min)
        .map(|(index, _)| index+1)
        .collect_vec();

    (ans.len(), ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!((2, vec![1, 4]), run(4, vec![vec![7, 19, 20], vec![4, 19, 24, 0], vec![26, 10], vec![19, 31, 24]], 19));
        assert_eq!((0, vec![0]), run(3, vec![vec![1], vec![2], vec![3]], 0));
    }
}
