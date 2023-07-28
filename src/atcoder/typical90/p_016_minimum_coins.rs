// https://atcoder.jp/contests/typical90/tasks/typical90_p

#[allow(dead_code)]
pub fn run(n: usize, a: usize, b: usize, c: usize) -> usize {
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

    #[test]
    fn test() {
        assert_eq!(5, run(227, 21, 47, 56));
        assert_eq!(1004, run(9999, 1, 5, 10));
        assert_eq!(3333, run(998244353, 314159, 265358, 97932));
        assert_eq!(9998, run(100000000, 10001, 10002, 10003));
    }
}
