// https://atcoder.jp/contests/dp/tasks/dp_a

pub fn run(n: i32, h: Vec<i32>) -> i32 {
    let mut dp = vec![std::i32::MAX; n as usize];

    dp[0] = 0;

    for i in 1..n {
        dp[i as usize] = dp[i as usize - 1] + ((h[i as usize] - h[i as usize - 1]) as i32).abs();

        if i > 1 {
            dp[i as usize] = (dp[i as usize]).min(dp[i as usize - 2] + ((h[i as usize] - h[i as usize - 2]) as i32).abs())
        }
    }

    dp[n as usize -1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(8, run(7, vec![2, 9, 4, 5, 1, 6, 10]));
        assert_eq!(30, run(4, vec![10, 30, 40, 20]));
        assert_eq!(0, run(2, vec![10, 10]));
        assert_eq!(40, run(6, vec![30, 10, 60, 10, 60, 50]));
    }
}
