// https://yukicoder.me/problems/no/3406

fn run(n: usize, a: usize, h: usize, m: usize, s: usize) -> &'static str {
    let current = h * 60 * 60 + m * 60 + s;

    let limit = 24 * 60 * 60;

    if current + n * a >= limit {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize, &'static str);

    #[test]
    fn yuki_3406() {
        let tests = [
            TestCase(108, 100, 23, 0, 0, "Yes"),
            TestCase(150, 300, 0, 0, 0, "No"),
            TestCase(150, 60, 21, 30, 0, "Yes"),
        ];

        for TestCase(n, a, h, m, s, expected) in tests {
            assert_eq!(run(n, a, h, m, s), expected);
        }
    }
}
