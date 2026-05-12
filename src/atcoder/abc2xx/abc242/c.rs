// https://atcoder.jp/contests/abc242/tasks/abc242_c

fn run(n: usize) -> usize {
    const MOD: usize = 998244353;

    let mut dp = vec![vec![0; 10]; n + 1];

    for i in 1..=9 {
        dp[1][i] = 1;
    }

    for i in 2..=n {
        for j in 1..=9 {
            if j > 1 {
                dp[i][j] += dp[i-1][j-1] % MOD;
            }

            dp[i][j] += dp[i-1][j] % MOD;

            if j < 9 {
                dp[i][j] += dp[i-1][j+1] % MOD;
            }

        }
    }

    dp[n].iter().sum::<usize>() % MOD
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn abc242_c() {
        let tests = [
            TestCase(4, 203),
            TestCase(2, 25),
            TestCase(1000000, 248860093),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
