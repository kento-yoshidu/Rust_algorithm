// https://yukicoder.me/problems/no/2736

fn run(a: usize, b: usize) -> &'static str {
    if a*2 < b || b*2 < a {
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
    fn yuki_2736() {
        let tests = [
            TestCase(6, 6, "Yes"),
            TestCase(4, 9, "No"),
            TestCase(500, 250, "Yes"),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
