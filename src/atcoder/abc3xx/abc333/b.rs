// https://atcoder.jp/contests/abc333/tasks/abc333_b

fn run(s: &str, t: &str) -> &'static str {
    let vec = ["AB", "BA", "BC", "CB", "CD", "DC", "DE", "ED", "EA", "AE"];

    if vec.contains(&s) == vec.contains(&t) {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, &'static str);

    #[test]
    fn abc333_b() {
        let tests = [
            TestCase("AC", "EC", "Yes"),
            TestCase("DA", "EA", "No"),
            TestCase("BD", "BD", "Yes"),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
