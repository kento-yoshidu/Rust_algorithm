// https://atcoder.jp/contests/abc129/tasks/abc129_c

fn run(n: usize, _m: usize, a: Vec<usize>) -> usize {
    const M: usize = 1_000_000_007;

    let mut broken = vec![false; n+1];

    for a in a {
        broken[a] = true;
    }

    let mut dp = vec![0; n+1];
    dp[0] = 1;

    if !broken[1] {
        dp[1] = 1;
    }

    for i in 2..=n {
        if !broken[i-1] {
            dp[i] += dp[i-1];
        }

        if !broken[i-2] {
            dp[i] += dp[i-2];
        }

        dp[i] %= M;
    }

    dp[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn abc129_c() {
        let tests = [
            TestCase(6, 1, vec![3], 4),
            TestCase(10, 2, vec![4, 5], 0),
            TestCase(100, 5, vec![1, 23, 45, 67, 89], 608200469),
        ];

        for TestCase(n, m, a, expected) in tests {
            assert_eq!(run(n, m, a), expected);
        }
    }
}
