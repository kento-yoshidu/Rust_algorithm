// https://atcoder.jp/contests/abc206/tasks/abc206_a

fn run(n: f64) -> &'static str {
    let price = (n * 1.08).floor() as usize;

    if price == 206 {
        "so-so"
    } else if price > 206 {
        ":("
    } else {
        "Yay!"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(f64, &'static str);

    #[test]
    fn abc206_a() {
        let tests = [
            TestCase(180.0, "Yay!"),
            TestCase(200.0, ":("),
            TestCase(191.0, "so-so"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
