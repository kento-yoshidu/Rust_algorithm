// https://atcoder.jp/contests/typical90/tasks/typical90_p

fn run(n: usize, a: usize, b: usize, c: usize) -> usize {
    let mut ans = std::usize::MAX;

    for a_number in 0..10000 {
        if a_number*a > n {
            break
        }

        for b_number in 0..10000 {
            if (a_number*a + b_number*b) > n {
                break
            }

            if (n - (a_number*a + b_number*b)) % c == 0 {
                let c_number = (n - (a_number*a + b_number*b)) / c;

                ans = ans.min(a_number+b_number+c_number);
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize);

    #[test]
    fn typical90_p() {
        let tests = [
            TestCase(227, 21, 47, 56, 5),
            TestCase(9999, 1, 5, 10, 1004),
            TestCase(998244353, 314159, 265358, 97932, 3333),
            TestCase(100000000, 10001, 10002, 10003, 9998),
        ];

        for TestCase(n, a, b, c, expected) in tests {
            assert_eq!(run(n, a, b, c), expected);
        }
    }
}
