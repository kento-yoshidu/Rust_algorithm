// https://atcoder.jp/contests/abc321/tasks/abc321_a

fn run(s: &str) -> &'static str {
    let chars: Vec<char> = s.chars().collect();

    for i in 1..chars.len() {
        if chars[i] >= chars[i-1] {
            return "No";
        }
    }

    "Yes"
}

fn run2(s: &str) -> &'static str {
    if s.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
        .windows(2)
        .all(|arr| {
            arr[0] > arr[1]
        }) {
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
    fn test() {
        let tests = [
            TestCase("321", "Yes"),
            TestCase("123", "No"),
            TestCase("1", "Yes"),
            TestCase("86411", "No"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
            assert_eq!(run2(s), expected);
        }
    }
}
