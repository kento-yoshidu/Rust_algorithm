// https://atcoder.jp/contests/abc183/tasks/abc183_b

pub fn run(sx: isize, sy: isize, gx: isize, gy: isize) -> f64 {
    ((sx * gy) + (sy * gx)) as f64 / (sy + gy) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3.0, run(1, 1, 7, 2));
        assert_eq!(1.6666666666666667, run(1, 1, 3, 2));
        assert_eq!(-18.705882352941178, run(-9, 99, -999, 9999));
    }
}
