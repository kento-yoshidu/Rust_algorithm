// https://atcoder.jp/contests/abc244/tasks/abc244_d

fn func(s: &str) -> bool {
    match s {
        "R G B" | "G B R" | "B R G" => true,
        _ => false,
    }
}

fn run(s: &str, t: &str) -> &'static str {
    if func(&s) == func(&t) {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, &'static str);

    #[test]
    fn abc244_c() {
        let tests = [
            TestCase("R G B", "R G B", "Yes"),
            TestCase("R G B", "R B G", "No"),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
