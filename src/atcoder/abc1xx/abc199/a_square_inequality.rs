// https://atcoder.jp/contests/abc199/tasks/abc199_a

fn run(a: usize, b: usize, c: usize) -> &'static str {
    if c*c > a*a + b*b {
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
    fn abc199_a() {
        let tests = [
            TestCase(2, 2, 4, "Yes"),
            TestCase(10, 10, 10, "No"),
            TestCase(3, 4, 5, "No"),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
