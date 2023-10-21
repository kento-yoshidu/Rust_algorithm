// https://atcoder.jp/contests/abc203/tasks/abc203_b

pub fn run(n: usize, k: usize) -> usize {
    let mut ans = 0;

    for nn in 1..=n {
        for kk in 1..=k {
            ans += nn*100 + kk
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(203, run(1, 2));
        assert_eq!(1818, run(3, 3));
    }
}
