// https://atcoder.jp/contests/abc266/tasks/abc266_b

pub fn run(n: isize) -> isize {
    let ans = n % 998244353;

    if ans < 0 {
        ans + 998244353
    } else {
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(998244354));
        assert_eq!(998244349, run(-9982443534));
        assert_eq!(0, run(-366387905869936605));
    }
}
