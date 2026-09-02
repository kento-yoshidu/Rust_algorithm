// https://atcoder.jp/contests/abc063/tasks/abc063_b

fn run(s: &str) -> &'static str {
    let mut chars: Vec<char> = s.chars().collect();

    chars.sort();

    if chars.windows(2).all(|v| {
        v[0] != v[1]
    }) {
        "yes"
    } else {
        "no"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc063_b() {
        let tests = [
            TestCase("uncopyrightable", "yes"),
            TestCase("different", "no"),
            TestCase("no", "yes"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
