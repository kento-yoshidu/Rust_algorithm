// https://atcoder.jp/contests/abc101/tasks/arc099_a

fn run(n: usize, k: usize, _a: &Vec<usize>) -> usize {
    ((n-1) as f64 / (k-1) as f64).ceil() as usize
}

fn run2(n: usize, k: usize, _a: &Vec<usize>) -> usize {
    let mut ans = 0;
    let mut right = 0;

    loop {
        if ans == 0 {
            right += k;
        } else {
            right += k - 1;
        }

        ans += 1;

        if right >= n {
            return ans;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn arc099_c() {
        let tests = [
            TestCase(4, 3, vec![2, 3, 1, 4], 2),
            TestCase(3, 3, vec![1, 2, 3], 1),
            TestCase(8, 3, vec![7, 3, 1, 8, 4, 6, 2, 5], 4),
        ];

        for TestCase(n, k, a, expected) in tests {
            assert_eq!(run(n, k, &a), expected);
            assert_eq!(run2(n, k, &a), expected);
        }
    }
}
