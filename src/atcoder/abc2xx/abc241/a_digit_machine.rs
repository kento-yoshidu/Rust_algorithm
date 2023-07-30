// https://atcoder.jp/contests/abc241/tasks/abc241_a

pub fn run(vec: Vec<usize>) -> usize {
    let mut ans = 0;

    for _ in 0..3 {
        ans = vec[ans];
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(7, run(vec![9, 0, 1, 2, 3, 4, 5, 6, 7, 8]));
        assert_eq!(4, run(vec![4, 8, 8, 8, 0, 8, 8, 8, 8, 8]));
        assert_eq!(0, run(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
    }
}
