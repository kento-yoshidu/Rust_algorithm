// https://atcoder.jp/contests/abc065/tasks/arc076_a

fn run(n: usize, m: usize) -> usize {
    if (n as isize - m as isize).abs() > 1 {
        return 0;
    }

    const M: usize = 1000000007;

    let mut ans = 1;

    for i in 1..=n {
        ans *= i;
        ans %= M;
    }

    for i in 1..=m {
        ans *= i;
        ans %= M;
    }

    if n == m {
        ans * 2 % M
    } else {
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 2, 8),
            TestCase(3, 2, 12),
            TestCase(1, 8, 0),
            TestCase(100000, 100000, 530123477),
        ];

        for TestCase(n, m, expected) in tests {
            assert_eq!(run(n, m), expected);
        }
    }
}
