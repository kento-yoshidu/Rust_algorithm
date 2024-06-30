// https://atcoder.jp/contests/zone2021/editorial/1192

pub fn run(_n: usize, d: usize, h: usize, dh: Vec<(usize, usize)>) -> f64 {
    let mut ans = 0.0;

    for (dd, hh) in dh.into_iter() {
        ans = f64::max(ans, hh as f64 - dd as f64 * ((h as f64 - hh as f64) / (d as f64 - dd as f64)));
    }

    ans
}

#[cfg(test)]
mod tests {
    use crate::atcoder::tessoku_book;

    use super::*;

    struct TestCase(usize, usize, usize, Vec<(usize, usize)>, f64);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 10, 10, vec![(3, 5)], 2.857142857142857),
            TestCase(1, 10, 10, vec![(3, 2)], 0.0),
            TestCase(5, 896, 483, vec![(228, 59), (529, 310), (339, 60), (78, 266), (659, 391)], 245.3080684596577),
        ];

        for TestCase(n, d, h, dh, expected) in tests {
            assert_eq!(run(n, d, h, dh), expected);
        }
    }
}
