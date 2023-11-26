// https://atcoder.jp/contests/abc330/tasks/abc330_b

pub fn run(_n: usize, l: usize, r: usize, a: Vec<usize>) -> Vec<usize> {
    a.iter()
        .map(|&num| {
            if l <= num && num <= r {
                num
            } else if num < l {
                l
            } else {
                r
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![4, 4, 4, 7, 7], run(5, 4, 7, vec![3, 1, 4, 9, 7]));
        assert_eq!(vec![10, 10, 10], run(3, 10, 10, vec![11, 10, 9]));
    }
}
