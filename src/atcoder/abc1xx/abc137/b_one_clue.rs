// https://atcoder.jp/contests/abc137/tasks/abc137_b

pub fn run(k: isize, x: isize) -> Vec<isize> {
    ((x-(k-1))..=(x+(k-1)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![5, 6, 7, 8, 9], run(3, 7));
        assert_eq!(vec![-3, -2, -1, 0, 1, 2, 3], run(4, 0));
        assert_eq!(vec![100], run(1, 100));
    }
}
