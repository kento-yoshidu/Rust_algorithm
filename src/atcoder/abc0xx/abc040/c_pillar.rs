// https://atcoder.jp/contests/abc040/tasks/abc040_c

pub fn run(n: i32, a: Vec<i32>) -> i32 {
    let mut dp = vec![std::i32::MAX; n as usize];

    dp[0] = 0;
    dp[1] = (a[1] - a[0]).abs();

    for i in 2..n {
        let tmp = dp[i as usize - 1] + ((a[i as usize] - a[i as usize - 1]) as i32).abs();
        let tmp2 = dp[i as usize - 2] + ((a[i as usize] - a[i as usize - 2]) as i32).abs();

        dp[i as usize] = tmp.min(tmp2);
    }

    dp[n as usize - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(40, run(4, vec![100, 150, 130, 120]));
        assert_eq!(40, run(4, vec![100, 125, 80, 110]));
        assert_eq!(310, run(9, vec![314, 159, 265, 358, 979, 323, 846, 264, 338]))
    }
}
