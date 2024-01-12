// https://atcoder.jp/contests/abc331/tasks/abc331_b

use itertools::iproduct;

pub fn run(n: usize, s: usize, m: usize, l: usize) -> usize {
    let mut ans = std::usize::MAX;

    for i in 0..=100 {
        for j in 0..=100 {
            for k in 0..=100 {
                if i*6 + j*8 + k*12 >= n {
                    ans = ans.min(i*s + j*m + k*l);
                }
            }

            if i*6 + j*8 > n {
                break;
            }
        }

        if i*6 > n {
            break
        }
    }

    ans
}

pub fn run2(n: usize, s: usize, m: usize, l: usize) -> usize {
    let mut ans = std::usize::MAX;

    for (i, j, k) in iproduct!(0..=100, 0..=100, 0..=100) {
        if i*6 + j*8 + k*12 >= n {
            ans = ans.min(i*s + j*m + k*l);
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(300, run(16, 120,  150, 200));
        assert_eq!(10, run(10, 100, 50, 10));
        assert_eq!(10000, run(99, 600, 800, 1200));
    }

    #[test]
    fn test2() {
        assert_eq!(300, run2(16, 120,  150, 200));
        assert_eq!(10, run2(10, 100, 50, 10));
        assert_eq!(10000, run2(99, 600, 800, 1200));
    }
}
