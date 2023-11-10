// https://atcoder.jp/contests/abc152/tasks/abc152_c

pub fn run(_n: usize, p: Vec<usize>) -> usize {
    let mut ans = 1;
    let mut min = p[0];

    for num in p {
        if num < min {
            min = num;
            ans += 1
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(5, vec![4, 2, 5, 1, 3]));
        assert_eq!(4, run(4, vec![4, 3, 2, 1]));
        assert_eq!(1, run(6, vec![1, 2, 3, 4, 5, 6]));
        assert_eq!(4, run(8, vec![5, 7, 4, 2, 6, 8, 1, 3]));
        assert_eq!(1, run(1, vec![1]));
    }
}
