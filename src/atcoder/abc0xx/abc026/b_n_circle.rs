// https://atcoder.jp/contests/abc026/tasks/abc026_b

use std::f64::consts::PI;

pub fn run(_n: i32, r: Vec<i32>) -> f64 {
    let mut vec = r.clone();

    vec.sort();
    vec.reverse();

    (vec.iter()
        .enumerate()
        .map(|(i, num)| {
            if i % 2 == 0 {
                num * num
            } else {
                -(num * num)
            }
        })
        .sum::<i32>()) as f64 * PI
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(18.84955592153876, run(3, vec![1, 2, 3]));
        assert_eq!(508.93800988154646, run(6, vec![15, 2, 3, 7, 6, 9]));
    }
}
