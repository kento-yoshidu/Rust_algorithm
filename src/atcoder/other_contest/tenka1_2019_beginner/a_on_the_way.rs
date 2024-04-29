// https://atcoder.jp/contests/tenka1-2019-beginner/tasks/tenka1_2019_a

fn run(a: usize, b: usize, c: usize) -> &'static str {
    if (a > c && c > b) || (a < c && c < b) {
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
            TestCase(3, 8, 5, "Yes"),
            TestCase(7, 3, 1, "No"),
            TestCase(10, 2, 4, "Yes"),
            TestCase(31, 41, 59, "No"),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
