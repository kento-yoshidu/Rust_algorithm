// https://atcoder.jp/contests/abc051/tasks/abc051_a

fn run(s: &str) -> String {
    s.replace(",", " ")
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("happy,newyear,enjoy", "happy newyear enjoy"),
            TestCase("haiku,atcoder,tasks", "haiku atcoder tasks"),
            TestCase("abcde,fghihgf,edcba", "abcde fghihgf edcba"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
