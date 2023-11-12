// https://atcoder.jp/contests/abc117/tasks/abc117_a

pub fn run(t: i32, x: i32) -> f64 {
    t as f64 / x as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2.6666666666666665, run(8, 3));
        assert_eq!(99.0000000000, run(99, 1));
        assert_eq!(0.01, run(1, 100));
    }
}
