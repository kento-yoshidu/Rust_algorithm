// https://atcoder.jp/contests/abc211/tasks/abc211_a

pub fn run(a: usize, b: usize) -> f64 {
    ((a - b) as f64 / 3.0) + b as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(110.0, run(130, 100));
        assert_eq!(133.33333333333331, run(300, 50));
        assert_eq!(123.0, run(123, 123));
    }
}
