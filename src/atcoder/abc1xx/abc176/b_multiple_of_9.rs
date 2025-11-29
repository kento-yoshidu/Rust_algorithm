// https://atcoder.jp/contests/abc176/tasks/abc176_b

pub fn run(n: &str) -> &'static str {
    let total: usize = n.chars()
        .map(|c| {
            c as usize - 48
        })
        .sum();

    if total % 9 == 0 {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc176_b() {
        let tests = [
            TestCase("123456789", "Yes"),
            TestCase("0", "Yes"),
            TestCase("31415926535897932384626433832795028841971693993751058209749445923078164062862089986280", "No"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
