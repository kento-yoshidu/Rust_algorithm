// https://atcoder.jp/contests/abc464/tasks/abc464_a

fn run(s: &str) -> &'static str {
    let e = s.chars().filter(|c| *c == 'E').count();
    let w = s.chars().filter(|c| *c == 'W').count();

    if e > w {
        "East"
    } else {
        "West"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc464_a() {
        let tests = [
            TestCase("EEWEW", "East"),
            TestCase("WWWWWWW", "West"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
