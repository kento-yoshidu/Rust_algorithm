use std::cmp::min;

pub fn run(n: usize, a: Vec<isize>, b: Vec<isize>) -> (usize, Vec<usize>) {
    let mut dp = vec![0];
    dp.push(a[0]);

    for i in 2..n {
        dp.push(min(dp[i-1] + a[i-1], dp[i-2] + b[i-2]));
    }

    let mut ans = Vec::new();

    let mut pos = n;

    ans.push(pos);

    loop {
        if pos == 1 {
            break;
        }

        if dp[pos-1] == dp[pos-2] + a[pos-2] {
            pos -= 1;
            ans.push(pos);
        } else {
            pos -= 2;
            ans.push(pos);
        }
    }

    (ans.len(), ans.into_iter().rev().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, Vec<isize>, (usize, Vec<usize>));

    #[test]
    fn test() {
        let tests = [
            TestCase(5, vec![2, 4, 1, 3], vec![5, 3, 7], (4, vec![1, 2, 4, 5])),
        ];

        for TestCase(n, a, b, expected) in tests {
            assert_eq!(run(n, a, b), expected);
        }
    }
}
