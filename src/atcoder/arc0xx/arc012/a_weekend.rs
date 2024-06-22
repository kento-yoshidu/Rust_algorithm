// https://atcoder.jp/contests/arc012/tasks/arc012_1

pub fn run(day: &str) -> usize {
    match day {
        "Friday" => 1,
        "Thursday" => 2,
        "Wednesday" => 3,
        "Tuesday" => 4,
        "Monday" => 5,
        "Saturday" | "Sunday" => 0,
        _ => unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase("Monday", 5),
            TestCase("Saturday", 0),
            TestCase("Sunday", 0),
            TestCase("Wednesday", 3),
        ];

        for TestCase(day, expected) in tests {
            assert_eq!(run(day), expected);
        }
    }
}
