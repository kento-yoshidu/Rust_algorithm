// https://atcoder.jp/contests/abc217/tasks/abc217_b

fn run(s: Vec<&str>) -> String {
    ["ABC", "ARC", "AGC", "AHC"]
        .into_iter()
        .find(|str| {
            !s.contains(str)
        })
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<&'static str>, &'static str);

    #[test]
    fn abc217_b() {
        let tests = [
            TestCase(vec!["ARC", "AGC", "AHC"], "ABC"),
            TestCase(vec!["AGC", "ABC", "ARC"], "AHC"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
