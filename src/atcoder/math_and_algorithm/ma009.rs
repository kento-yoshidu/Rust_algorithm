// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_i

fn run(n: usize, s: usize, a: Vec<usize>) -> &'static str {
    let mut dp = vec![false; s+1];

    dp[0] = true;

    for i in 0..n {
        for j in (0..s).rev() {
            if dp[j] && j + a[i] <= s {
                dp[j + a[i]] = true;
            }
        }
    }

    if dp[s] {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, &'static str);

    #[test]
    fn ma009() {
        let tests = [
            TestCase(3, 11, vec![2, 5, 9], "Yes"),
            TestCase(4, 11, vec![3, 1, 4, 5], "No"),
        ];

        for TestCase(n, s, a, expected) in tests {
            assert_eq!(run(n, s, a), expected);
        }
    }
}
