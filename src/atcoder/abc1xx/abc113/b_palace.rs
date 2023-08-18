// https://atcoder.jp/contests/abc113/tasks/abc113_b

use itertools::Itertools;

pub fn run(n: i32, t: i32, a: i32, vec: Vec<i32>) -> usize {
    let mut tmp: Vec<i32> = Vec::new();

    for i in 0..n {
        let num = t as f64 - vec[i as usize] as f64 * 0.006;

        tmp.push((a - num as i32).abs())
    }

    tmp.iter().position_min().unwrap() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(2, 12, 5, vec![1000, 2000]));
        assert_eq!(3, run(3, 21, -11, vec![81234, 94124, 52141, 2000]));
    }
}
