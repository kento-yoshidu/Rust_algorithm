// https://yukicoder.me/problems/no/3248

fn run(n: usize, _x: usize) -> &'static str {
    if n == 1 {
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
    fn yuki_3248() {
        let tests = [
            TestCase(1, 1, "No"),
            TestCase(2, 1, "Yes"),
        ];

        for TestCase(n, x, expected) in tests {
            assert_eq!(run(n, x), expected);
        }
    }
}
