// https://atcoder.jp/contests/abc239/tasks/abc239_a

fn run(h: f64) -> f64 {
    ((h * (h + 12800000.0))).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(f64, f64);

    #[test]
    fn abc239_a() {
        let tests = [
            TestCase(333.0, 65287.9076782217),
            TestCase(634.0,90086.63583462311),
        ];

        for TestCase(h, expected) in tests {
            assert_eq!(run(h), expected);
        }
    }
}
