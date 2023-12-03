// https://atcoder.jp/contests/abc112/tasks/abc112_b

pub fn run(_n: usize, t: usize, vec: Vec<(usize, usize)>) -> String {
    vec.iter()
        .filter(|tup| {
            tup.1 <= t
        })
        .min_by(|a, b| {
            a.0.cmp(&b.0)
        })
        .map(|tup| {
            tup.0.to_string()
        })
        .unwrap_or("TLE".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("4"), run(3, 70, vec![(7, 60), (1, 80), (4, 50)]));
        assert_eq!(String::from("TLE"), run(4, 3, vec![(1, 1000), (2, 4), (3, 1000), (4, 500)]));
        assert_eq!(String::from("5"), run(5, 9, vec![(25, 8), (5, 9), (4, 10), (1000, 1000), (6, 1)]));
    }
}