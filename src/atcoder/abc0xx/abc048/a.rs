// https://atcoder.jp/contests/abc048/tasks/abc048_a

fn run(a: &str, b: &str, c: &str) -> String {
    let vec = vec![a, b, c];

    vec.into_iter()
        .map(|v| {
            v.chars().nth(0).unwrap()
        })
        .collect()
}

fn run2(_a: &str, b: &str, _c: &str) -> String {
    format!("A{}C", b.chars().nth(0).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, &'static str, &'static str);

    #[test]
    fn abc048_a() {
        let tests = [
            TestCase("AtCoder", "Beginner", "Contest", "ABC"),
            TestCase("AtCoder", "Snuke", "Contest", "ASC"),
            TestCase("AtCoder", "X", "Contest", "AXC"),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
            assert_eq!(run2(a, b, c), expected);
        }
    }
}
