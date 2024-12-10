// https://atcoder.jp/contests/abc051/tasks/abc051_b

fn run(k: usize, s: usize) -> usize {
    let mut ans = 0;

    for x in 0..=k {
        for y in 0..=k {
            if x + y > s {
                break
            }

            if x + y == s {
                ans += 1;
                break
            }

            if (s - x - y) <= k {
                ans += 1
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 2, 6),
            TestCase(5, 15, 1),
        ];

        for TestCase(k, s, expected) in tests {
            assert_eq!(run(k, s), expected);
        }
    }
}
