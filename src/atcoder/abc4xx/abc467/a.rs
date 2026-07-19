// https://atcoder.jp/contests/abc467/tasks/abc467_a

fn run(h: usize, w: usize) -> &'static str {
    if 400 * w >= h * h {
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
    fn abc467_a() {
        let tests = [
            TestCase(180, 60, "No"),
            TestCase(182, 188, "Yes"),
            TestCase(180, 81, "Yes"),
        ];

        for TestCase(h, w, expected) in tests {
            assert_eq!(run(h, w), expected);
        }
    }
}
