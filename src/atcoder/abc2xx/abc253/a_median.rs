// https://atcoder.jp/contests/abc254/tasks/abc254_a

fn run(a: usize, b: usize, c: usize) -> &'static str {
    if a <= b && b <= c || a >= b && b >= c {
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
    fn abc253_a() {
        let tests = [
            TestCase(5, 3, 2, "Yes"),
            TestCase(2, 5, 3, "No"),
            TestCase(100, 100, 100, "Yes"),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
