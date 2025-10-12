// https://atcoder.jp/contests/abc166/tasks/abc166_a

pub fn run(s: &str) -> &'static str {
    match s {
        "ABC" => "ARC",
        "ARC" => "ABC",
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc166_a() {
        let tests = [
            TestCase("ABC", "ARC"),
            TestCase("ARC" ,"ABC"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
