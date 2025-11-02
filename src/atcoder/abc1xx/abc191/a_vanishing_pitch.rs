// https://atcoder.jp/contests/abc191/tasks/abc191_a

fn run(v: isize, t: isize, s: isize, d: isize) -> &'static str {
    if d < v*t || v*s < d {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize, &'static str);

    #[test]
    fn abc191_a() {
        let tests = [
            TestCase(10, 3, 5, 20, "Yes"),
            TestCase(10, 3, 5, 30, "No"),
        ];

        for TestCase(v, t, s, d, expected) in tests {
            assert_eq!(run(v, t, s, d), expected);
        }
    }
}
