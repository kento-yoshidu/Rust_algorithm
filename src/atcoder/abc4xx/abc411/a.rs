// https://atcoder.jp/contests/abc411/tasks/abc411_a

fn run(p: &str, l: usize) -> &'static str {
    if p.len() >= l {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize, &'static str);

    #[test]
    fn abc411_a() {
        let tests = [
            TestCase("chokudai", 5, "Yes"),
            TestCase("ac", 3, "No"),
            TestCase("atcoder", 7, "Yes"),
        ];

        for TestCase(p, l, expected) in tests {
            assert_eq!(run(p, l), expected);
        }
    }
}
