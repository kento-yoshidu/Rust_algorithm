// https://atcoder.jp/contests/abc179/tasks/abc179_c

fn run(n: usize) -> usize {
    let mut ans = 0;

    for a in 1..n {
        for b in 1..n {
            if a*b >= n {
                break
            }

            let num = n - (a*b);

            if num >= n {
                break
            } else {
                ans += 1;
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn abc179_c() {
        let tests = [
            TestCase(3, 3),
            TestCase(100, 473),
            TestCase(1000000, 13969985),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
