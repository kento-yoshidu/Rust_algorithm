// https://yukicoder.me/problems/no/3124

fn run(x: usize, y: usize) -> &'static str {
    if x == y {
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
    fn yuki_3124() {
        let tests = [
            TestCase(10, 10, "Yes"),
            TestCase(15, 18, "No"),
            TestCase(425, 425, "Yes"),
        ];

        for TestCase(x, y, expected) in tests {
            assert_eq!(run(x, y), expected);
        }
    }
}
