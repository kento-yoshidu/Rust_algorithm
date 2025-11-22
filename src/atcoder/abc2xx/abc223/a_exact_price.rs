// https://atcoder.jp/contests/abc223/tasks/abc223_a

fn run(x: usize) -> &'static str {
    if x % 100 == 0 && x != 0 {
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
    fn abc223_a() {
        let tests = [
            TestCase(500, "Yes"),
            TestCase(40, "No"),
            TestCase(0, "No"),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
