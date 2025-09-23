// https://atcoder.jp/contests/abc146/tasks/abc146_a

fn run(s: &str) -> usize {
    let arr = ["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];

    arr.into_iter()
        .rev()
        .position(|day| s == day)
        .unwrap() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn abc146_a() {
        let tests = [
            TestCase("SAT", 1),
            TestCase("FRI", 2),
            TestCase("THU", 3),
            TestCase("WED", 4),
            TestCase("TUE", 5),
            TestCase("MON", 6),
            TestCase("SUN", 7),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
