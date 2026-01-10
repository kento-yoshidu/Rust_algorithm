// https://atcoder.jp/contests/abc229/tasks/abc229_a

fn run(s1: &str, s2: &str) -> &'static str {
    let s1_count = s1.chars().filter(|c| *c == '#').count();
    let s2_count = s2.chars().filter(|c| *c == '#').count();

    if s1_count + s2_count >= 3 {
        return "Yes";
    }

    if s1_count == 2 || s2_count == 2 {
        return "Yes";
    }

    if s1
        .chars()
        .zip(s2.chars())
        .any(|v| {
            v.0 == '#' && v.1 == '#'
        }) {
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
    fn abc229_a() {
        let tests = [
            TestCase("##", "#.", "Yes"),
            TestCase(".#", "#.", "No"),
            TestCase("##", "..", "Yes"),
        ];

        for TestCase(s1, s2, expected) in tests {
            assert_eq!(run(s1, s2), expected);
        }
    }
}
