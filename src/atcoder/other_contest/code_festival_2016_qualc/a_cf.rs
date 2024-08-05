// https://atcoder.jp/contests/code-festival-2016-qualc/tasks/codefestival_2016_qualC_a

fn run(s: &str) -> &'static str {
    let Some(c) = s.find(|c| c == 'C') else {
        return "No";
    };

    let Some(f) = s.rfind(|c| c == 'F') else {
        return "No";
    };

    if c < f {
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
            TestCase("CODEFESTIVAL", "Yes"),
            TestCase("FESTIVALCODE", "No"),
            TestCase("CF", "Yes"),
            TestCase("FCF", "Yes")
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
