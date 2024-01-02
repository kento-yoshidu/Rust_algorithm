// https://atcoder.jp/contests/abc304/tasks/abc304_b

pub fn run(n: usize) -> usize {
    // 桁数を求める
    let mut len = n.to_string().len();

    if len <= 3 {
        return n;
    } else {
        len -= 3;
    }

    let tmp = 10_i32.pow(len as u32);

    (n / tmp as usize) * tmp as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(20200000, run(20230603));
        assert_eq!(0, run(0));
        assert_eq!(304, run(304));
        assert_eq!(500000, run(500600));
    }
}
