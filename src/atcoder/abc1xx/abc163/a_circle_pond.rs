// https://atcoder.jp/contests/abc163/tasks/abc163_a

pub fn run(r: usize) -> f64 {
    r as f64 * 2.0 * std::f64::consts::PI
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(6.28318530717958623200, run(1));
        assert_eq!(458.67252742410977361942, run(73));
    }
}
