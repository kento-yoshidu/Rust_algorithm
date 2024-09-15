// https://atcoder.jp/contests/tenka1-2014-qualb/tasks/tenka1_2014_qualB_a

fn run(s: &str) -> String {
    s.replace("HAGIYA", "HAGIXILE")
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("MRHAGIYA", "MRHAGIXILE"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
