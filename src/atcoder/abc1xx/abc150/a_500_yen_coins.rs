// https://atcoder.jp/contests/abc150/tasks/abc150_a

fn run(k: usize, x: usize) -> &'static str {
    if k * 500 >= x {
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
    fn abc150_a() {
        let tests = [
            TestCase(2, 900, "Yes"),
            TestCase(1, 501, "No"),
            TestCase(4, 2000, "Yes"),
        ];

        for TestCase(k, x, expected) in tests {
            assert_eq!(run(k, x), expected);
        }
    }
}
