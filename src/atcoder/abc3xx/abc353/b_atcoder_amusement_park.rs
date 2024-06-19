// https://atcoder.jp/contests/abc353/tasks/abc353_b

pub fn run(_n: usize, k: usize, a: Vec<usize>) -> usize {
    let mut ans = 0;

    let mut cnt = 0;

    for i in a {
        if cnt + i == k {
            ans += 1;
            cnt = 0;
        } else if cnt + i > k {
            ans += 1;
            cnt = i
        } else {
            cnt += i;
        }
    }

    if cnt != 0 {
        ans += 1;
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
            TestCase(7, 6, vec![2, 5, 1, 4, 1, 2, 3], 4),
            TestCase(7, 10, vec![1, 10, 1, 10, 1, 10, 1], 7),
            TestCase(15, 100, vec![73, 8, 55, 26, 97, 48, 37, 47, 35, 55, 5, 17, 62, 2, 60], 8),
        ];

        for TestCase(n, k, a, expected) in tests {
            assert_eq!(run(n, k, a), expected);
        }
    }
}
