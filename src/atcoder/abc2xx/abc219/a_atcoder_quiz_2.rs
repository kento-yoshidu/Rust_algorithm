// https://atcoder.jp/contests/abc219/tasks/abc219_a

pub fn run(x: usize) -> String {
    match x {
        0..=39 => (40 - x).to_string(),
        40..=69 => (70 - x).to_string(),
        70..=89 => (90 - x).to_string(),
        _ => String::from("expert"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn abc219_a() {
        let tests = [
            TestCase(56, "14"),
            TestCase(32, "8"),
            TestCase(0, "40"),
            TestCase(100, "expert"),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
