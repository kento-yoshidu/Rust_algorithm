// https://atcoder.jp/contests/abc246/tasks/abc246_b

pub fn run(a: i32, b: i32) -> (f64, f64) {
    let d = ((a.pow(2) + b.pow(2)) as f64).sqrt();

    (a as f64 / d, b as f64 / d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!((0.600000000000, 0.800000000000), run(3, 4));
        assert_eq!((1.0, 0.0), run(1, 0));
        assert_eq!((0.5219648702449755, 0.8529669830832527), run(246, 402));
    }
}
