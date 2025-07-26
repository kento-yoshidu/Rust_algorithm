// https://atcoder.jp/contests/abc119/tasks/abc119_b

fn run(_n: usize, vec: Vec<(f32, &str)>) -> f32 {
    vec.into_iter()
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

    struct TestCase(usize, Vec<(f32, &'static str)>, f32);

    #[test]
    fn abc119_b() {
        let tests = [
            TestCase(2, vec![(10000.0, "JPY"), (0.10000000, "BTC")], 48000.0),
            TestCase(3, vec![(100000000.0, "JPY"), (100.00000000, "BTC"), (0.00000001, "BTC")], 138000000.0038),
        ];

        for TestCase(n, vec, expected) in tests {
            assert_eq!(run(n, vec), expected);
        }
    }
}
