// https://atcoder.jp/contests/abc017/tasks/abc017_1

#[allow(dead_code)]
pub fn run(vec: Vec<(usize, usize)>) -> usize {
    vec.iter().map(|v| {
        v.0 as f64 * (v.1 as f64) * 0.1
    }).sum::<f64>() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(94, run(vec![(50, 7), (40, 8), (30, 9)]));
        assert_eq!(2970, run(vec![(990, 10), (990, 10), (990, 10)]));
    }
}
