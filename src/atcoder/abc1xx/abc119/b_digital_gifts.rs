// https://atcoder.jp/contests/abc119/tasks/abc119_b

pub fn run(_n: usize, vec: Vec<(f32, &str)>) -> f32 {
    vec.iter()
        .map(|t| {
            match t.1 {
                "JPY" => t.0,
                "BTC" => t.0 * 380000.0,
                _ => unreachable!(),
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(48000.0, run(2, vec![(10000.0, "JPY"), (0.10000000, "BTC")]));
        assert_eq!(138000000.0038, run(3, vec![(100000000.0, "JPY"), (100.00000000, "BTC"), (0.00000001, "BTC")]));
    }
}

