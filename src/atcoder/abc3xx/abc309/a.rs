// https://atcoder.jp/contests/abc309/tasks/abc309_a

fn run(a: usize, b: usize) -> &'static str {
    if a % 3 == 0 {
        "No"
    } else if b - a != 1 {
        "No"
    } else {
        "Yes"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str);

    #[test]
    fn abc309_a() {
        let tests = [
            TestCase(7, 8, "Yes"),
            TestCase(1, 9, "No"),
            TestCase(3, 4, "No"),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
