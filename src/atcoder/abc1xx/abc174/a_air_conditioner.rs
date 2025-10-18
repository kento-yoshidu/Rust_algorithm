// https://atcoder.jp/contests/abc174/tasks/abc174_a

fn run(x: usize) -> &'static str {
    if x >= 30 {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn abc174_a() {
        let tests = [
            TestCase(30, "Yes"),
            TestCase(25, "No"),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
