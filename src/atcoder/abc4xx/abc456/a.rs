// https://atcoder.jp/contests/abc456/tasks/abc456_a

fn run(x: usize) -> &'static str {
    if 3 <= x && x <= 18 {
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
    fn abc456_a() {
        let tests = [
            TestCase(15, "Yes"),
            TestCase(2, "No"),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
