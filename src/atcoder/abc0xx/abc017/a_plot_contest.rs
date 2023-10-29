// https://atcoder.jp/contests/abc017/tasks/abc017_1

pub fn run(vec: Vec<(usize, usize)>) -> usize {
    vec.iter().map(|v| {
        v.0 as f64 * (v.1 as f64) * 0.1
    }).sum::<f64>() as usize
}

pub fn run2(vec: Vec<(usize, usize)>) -> usize {
    vec.iter()
        .map(|t| {
            t.0 * t.1
        })
        .sum::<usize>() / 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(94, run(vec![(50, 7), (40, 8), (30, 9)]));
        assert_eq!(2970, run(vec![(990, 10), (990, 10), (990, 10)]));
    }

    #[test]
    fn test2() {
        assert_eq!(94, run2(vec![(50, 7), (40, 8), (30, 9)]));
        assert_eq!(2970, run2(vec![(990, 10), (990, 10), (990, 10)]));
    }
}
