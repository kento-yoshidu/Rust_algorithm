// https://atcoder.jp/contests/abc240/tasks/abc240_a

fn run(a: isize, b: isize) -> &'static str {
    if (a - b).abs() == 1 || (a - b).abs() == 9 {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, &'static str);

    #[test]
    fn abc240_a() {
        let tests = [
            TestCase(2, 3, "Yes"),
            TestCase(3, 5, "No"),
            TestCase(1, 10, "Yes"),
            TestCase(1, 2, "Yes"),
            TestCase(2, 3, "Yes"),
            TestCase(3, 4, "Yes"),
            TestCase(4, 5, "Yes"),
            TestCase(5, 6, "Yes"),
            TestCase(1, 5, "No"),
            TestCase(3, 9, "No"),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
