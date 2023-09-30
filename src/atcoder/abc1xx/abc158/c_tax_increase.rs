// https://atcoder.jp/contests/abc158/tasks/abc158_c

pub fn run(a: f64, b: f64) -> i32 {
    for i  in 10..=10000 {
        let tmp = f64::from(i);

        if (tmp * 0.08).floor() == a && (tmp * 0.1).floor() == b {
            return i;
        }
    }

    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(25, run(2.0, 2.0));
        assert_eq!(100, run(8.0, 10.0));
        assert_eq!(-1, run(19.0, 99.0));
    }
}
