// https://atcoder.jp/contests/panasonic2020/tasks/panasonic2020_c

pub fn run(a: usize, b: usize, c: usize) -> &'static str {
    if a + b < c && 4 * a * b < (c - a - b).pow(2) {
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
    fn test() {
        let tests = [
            TestCase(2, 3, 9, "No"),
            TestCase(2, 3, 10, "Yes"),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
