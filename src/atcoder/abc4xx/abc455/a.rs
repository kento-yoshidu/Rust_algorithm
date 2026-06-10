// https://atcoder.jp/contests/abc455/tasks/abc455_a

fn run(a: usize, b: usize, c: usize) -> &'static str {
    if a != b && b == c {
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
    fn abc455_a() {
        let tests = [
            TestCase(4, 5, 5, "Yes"),
            TestCase(1, 3, 7, "No"),
            TestCase(6, 6, 6, "No"),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
