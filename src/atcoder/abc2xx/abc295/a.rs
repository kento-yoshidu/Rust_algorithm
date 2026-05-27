// https://atcoder.jp/contests/abc295/tasks/abc295_a

fn run(str: &str) -> &'static str {
    for word in ["and", "not", "that", "the", "you"] {
        if str.contains(word) {
            return "Yes";
        }
    }

    "No"
}

fn run2(s: &str) -> &'static str {
    if s.split(' ').any(|str| {
        ["and", "not", "that", "the", "you"].contains(&&str)
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
    fn abc295_a() {
        let tests = [
            TestCase("in that case you should print yes and not no", "Yes"),
            TestCase("in diesem fall sollten sie no und nicht yes ausgeben", "No"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
            assert_eq!(run2(s), expected);
        }
    }
}
