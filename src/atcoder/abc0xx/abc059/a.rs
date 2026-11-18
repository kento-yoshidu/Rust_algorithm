// https://atcoder.jp/contests/abc059/tasks/abc059_a

fn run(a: &str, b: &str, c: &str) -> String {
    let vec = vec![a, b, c];

    vec
        .iter()
        .map(|vec| {
            vec.chars().nth(0).unwrap()
        })
        .collect::<String>()
        .to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, &'static str, &'static str);

    #[test]
    fn abc059_a() {
        let tests = [
            TestCase("atcoder", "beginner", "contest", "ABC"),
            TestCase("resident", "register", "number", "RRN"),
            TestCase("k", "nearest", "neighbor", "KNN"),
            TestCase("async", "layered", "coding", "ALC"),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
