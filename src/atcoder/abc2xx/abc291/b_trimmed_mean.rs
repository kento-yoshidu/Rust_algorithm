// https://atcoder.jp/contests/abc291/tasks/abc291_b

use itertools::Itertools;

fn run(n: usize, x: Vec<usize>) -> f64 {
    x.iter()
        .sorted()
        .enumerate()
        .filter(|(i, _)| {
            !(i < &n || x.len() - n <= *i)
        })
        .map(|(_, num)| *num as f64)
        .sum::<f64>() / (x.len() - n*2) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abc291_b() {
        assert_eq!(33.333333333333336, run(1, vec![10, 100, 20, 50, 30]));
        assert_eq!(5.5, run(2, vec![3, 3, 3, 4, 5, 6, 7, 8, 99, 100]));
    }
}
