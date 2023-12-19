// https://atcoder.jp/contests/abc138/tasks/abc138_b

pub fn run(_n: usize, a: Vec<usize>) -> f64 {
    let vec: Vec<f64> = a.iter().map(|num| *num as f64).collect();

    let total: f64 = vec.iter()
        .map(|num| {
            1.0 / num
        })
        .sum();

    1.0 / total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(7.5, run(2, vec![10, 30]));
        assert_eq!(66.66666666666667, run(3, vec![200, 200, 200]));
        assert_eq!(1000.0, run(1, vec![1000]));
    }
}
