// https://atcoder.jp/contests/abc215/tasks/abc215_b

pub fn run(n: u128) -> u32 {
    let mut ans = 1;

    loop {
        if 2_u128.pow(ans) > n {
            return ans - 1;
        } else {
            ans += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(6));
        assert_eq!(0, run(1));
        assert_eq!(59, run(1000000000000000000));
        assert_eq!(59, run(576460752303423488));
    }
}
