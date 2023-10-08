// https://atcoder.jp/contests/abc290/tasks/abc290_a

pub fn run(_n: usize, m: usize, a: Vec<usize>, b: Vec<usize>) -> usize {
    let mut ans = 0;

    for i in 0..m {
        ans += a[b[i] - 1]
    }

    ans
}

pub fn run2(_n: usize, _m: usize, a: Vec<usize>, b: Vec<usize>) -> usize {
    b.iter().fold(0, |sum, &i| {
        sum + a[i - 1]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(40, run(3, 2, vec![10, 20, 30], vec![1, 3]));
        assert_eq!(100, run(4, 1, vec![1, 1, 1, 100], vec![4]));
        assert_eq!(202, run(8, 4, vec![22, 75, 26, 45, 72, 81, 47, 29], vec![4, 6, 7, 8]));
    }

    #[test]
    fn test2() {
        assert_eq!(40, run2(3, 2, vec![10, 20, 30], vec![1, 3]));
        assert_eq!(100, run2(4, 1, vec![1, 1, 1, 100], vec![4]));
        assert_eq!(202, run2(8, 4, vec![22, 75, 26, 45, 72, 81, 47, 29], vec![4, 6, 7, 8]));
    }
}
