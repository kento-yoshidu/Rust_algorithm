// https://atcoder.jp/contests/abc465/tasks/abc465_a

fn run(a: usize, b: usize) -> &'static str {
    if a as f64 > b as f64 * 2.0 / 3.0 {
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
    fn abc465_a() {
        let tests = [
            TestCase(316, 465, "Yes"),
            TestCase(101, 248, "No"),
            TestCase(666, 999, "No"),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
