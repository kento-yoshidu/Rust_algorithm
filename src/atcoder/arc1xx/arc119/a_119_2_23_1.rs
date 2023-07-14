// https://atcoder.jp/contests/arc119/tasks/arc119_a

#[allow(dead_code)]
pub fn run(n: i64) -> i64 {
    let mut ans = n;

    for b in 0..=60 {
        let mut c = n;
        let a = n/(2_i64.pow(b));
        c -= a * (2_i64.pow(b));

        let total = a + (b as i64) + c;

        ans = ans.min(total);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(143, run(998244353));
        assert_eq!(49483, run(1000000007));
        assert_eq!(1, run(1));
        assert_eq!(2003450165, run(998984374864432412));
    }
}
