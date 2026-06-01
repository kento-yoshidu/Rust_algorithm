// https://atcoder.jp/contests/tdpc/tasks/tdpc_contest

fn run(n: usize, p: Vec<usize>) -> usize {
    let len: usize = p.iter().sum();

    let mut dp = vec![vec![false; len+1]; n+1];
    dp[0][0] = true;

    for i in 1..=n {
        let p = p[i - 1];

        for j in 0..=len {
            dp[i][j] = dp[i-1][j];

            if p <= j {
                if dp[i-1][j - p] {
                    dp[i][j] = true;
                }
            }
        }
    }

    dp[n].iter().filter(|b| **b).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn tdpc_a() {
        let tests = [
            TestCase(3, vec![2, 3, 5], 7),
        ];

        for TestCase(n, p, expected) in tests {
            assert_eq!(run(n, p), expected);
        }
    }
}
