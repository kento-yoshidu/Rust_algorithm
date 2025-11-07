// https://atcoder.jp/contests/abc195/tasks/abc195_a

fn run(m: usize, h: usize) -> &'static str {
    if h % m == 0 {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str);

    #[test]
    fn abc194_a() {
        let tests = [
            TestCase(10, 120, "Yes"),
            TestCase(10, 125, "No"),
        ];

        for TestCase(m, h, expected) in tests {
            assert_eq!(run(m, h), expected);
        }
    }
}
