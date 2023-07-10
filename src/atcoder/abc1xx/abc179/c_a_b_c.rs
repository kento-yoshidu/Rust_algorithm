// https://atcoder.jp/contests/abc179/tasks/abc179_c

#[allow(dead_code)]
pub fn run(n: usize) -> usize {
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

    #[test]
    fn test() {
        assert_eq!(3, run(3));
        assert_eq!(473, run(100));
        assert_eq!(13969985, run(1000000));
    }
}
