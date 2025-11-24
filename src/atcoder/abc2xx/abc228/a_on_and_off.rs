// https://atcoder.jp/contests/abc228/tasks/abc228_a

fn run(s: usize, t: usize, x: usize) -> &'static str {
    if (s..)
        .take_while(|time| time % 24 != t)
        .any(|time| time % 24 == x)
    {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, &'static str);

    #[test]
    fn abc228_a() {
        let tests = [
            TestCase(7, 20, 12, "Yes"),
            TestCase(20, 7, 12, "No"),
            TestCase(23, 0, 23, "Yes"),
        ];

        for TestCase(s, t, x, expected) in tests {
            assert_eq!(run(s, t, x), expected);
        }
    }
}
