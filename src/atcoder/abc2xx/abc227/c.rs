// https://atcoder.jp/contests/abc227/tasks/abc227_c

fn run(n: usize) -> usize {
    let mut ans = 0;

    for a in 1..=n {
        if a * a * a > n {
            break;
        }

        for b in a..=n {
            if a * b * b > n {
                break;
            }

            let c_count = n / a / b;

            if c_count >= b {
                ans += c_count - b + 1;
            } else {
                break;
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
    fn abc227_c() {
        let tests = [
            TestCase(4, 5),
            TestCase(100, 323),
            TestCase(100000000000, 5745290566750),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
