// https://atcoder.jp/contests/abc160/tasks/abc160_a

fn run(s: &str) -> &'static str {
    let t = &s.chars().nth(2).unwrap();
    let fo = &s.chars().nth(3).unwrap();
    let fi = &s.chars().nth(4).unwrap();
    let s = &s.chars().nth(5).unwrap();

    if t == fo && fi == s {
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
    fn abc160_a() {
        let tests = [
            TestCase("sippuu", "Yes"),
            TestCase("iphone", "No"),
            TestCase("coffee", "Yes"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
