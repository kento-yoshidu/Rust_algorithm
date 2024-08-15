// https://atcoder.jp/contests/arc018/tasks/arc018_1

fn run(height: f64, bmi: f64) -> f64 {
    bmi * height.powf(2.0) / 10000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(f64, f64, f64);

    #[test]
    fn test() {
        let tests = [
            TestCase(160.0, 23.5, 60.16),
            TestCase(199.9, 11.1, 44.355611100000004),
        ];

        for TestCase(height, bmi, expected) in tests {
            assert_eq!(run(height, bmi), expected);
        }
    }
}
