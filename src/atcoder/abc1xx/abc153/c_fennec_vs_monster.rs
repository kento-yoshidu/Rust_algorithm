// https://atcoder.jp/contests/abc153/tasks/abc153_c

pub fn run(_n: usize, k: usize, h: Vec<usize>) -> usize {
    let mut vec = h.clone();

    vec.sort();
    vec.reverse();

    vec.iter()
        .skip(k)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5, run(3, 1, vec![4, 1, 5]));
        assert_eq!(0, run(8, 9, vec![7, 9, 3, 2, 3, 8, 4, 6]));
        assert_eq!(3000000000, run(3, 0, vec![1000000000, 1000000000, 1000000000]));
    }
}
