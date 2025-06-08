// https://atcoder.jp/contests/abc080/tasks/abc080_b

fn calc(n: String) -> u32 {
    n.chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum()
}

fn run(n: usize) -> &'static str {
    let num = calc(n.to_string()) as usize;

    if n % num == 0 {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(12, "Yes"),
            TestCase(57, "No"),
            TestCase(148, "No"),
            TestCase(27, "Yes"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
