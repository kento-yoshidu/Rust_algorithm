// https://atcoder.jp/contests/abc032/tasks/abc032_c

fn run(n: usize, k: usize, s: Vec<usize>) -> usize {
    if s.contains(&0) {
        return n
    }

    let mut left = 0;
    let mut right = 0;

    let mut current = 1;

    let mut ans = 0;

    while right < n {
        if current * s[right] <= k {
            current *= s[right];
            right += 1;
            ans = ans.max(right - left);
        } else if left == right {
            left += 1;
            right += 1;
        } else {
            current /= s[left];
            left += 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(7, 6, vec![4, 3, 1, 1, 2, 10, 2], 4),
            TestCase(6, 10, vec![10, 10, 10, 10, 0, 10], 6),
            TestCase(6, 9, vec![10, 10, 10, 10, 10, 10], 0),
            TestCase(4, 0, vec![1, 2, 3, 4], 0),
        ];

        for TestCase(n, k, s, expected) in tests {
            assert_eq!(run(n, k, s), expected);
        }
    }
}
