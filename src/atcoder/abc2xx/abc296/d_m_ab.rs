// https://atcoder.jp/contests/abc296/tasks/abc296_d

fn run(n: isize, m: isize) -> isize {
    let mut ans = std::isize::MAX;

    for a in 1..=n {
        let b = (m + a - 1) / a;

        if b <= n {
            ans = ans.min(a*b)
        }

        if a*a > m {
            break
        }
    }

    if ans == std::isize::MAX {
        -1
    } else {
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 7, 8),
            TestCase(2, 5, -1),
            TestCase(100000, 10000000000, 10000000000),
        ];

        for TestCase(n, m, expected) in tests {
            assert_eq!(run(n, m), expected);
        }
    }
}
