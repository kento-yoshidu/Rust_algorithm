// https://atcoder.jp/contests/abc188/tasks/abc188_a

fn run(x: isize, y: isize) -> &'static str {
    if (x - y).abs() <= 2 {
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
    fn abc188_a() {
        let tests = [
            TestCase(3, 5, "Yes"),
            TestCase(16, 2, "No"),
            TestCase(12, 15, "No"),
        ];

        for TestCase(x, y, expected) in tests {
            assert_eq!(run(x, y), expected);
        }
    }
}
