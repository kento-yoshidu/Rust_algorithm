// https://atcoder.jp/contests/abc059/tasks/abc059_a

fn run(a: &str, b: &str, c: &str) -> String {
    let vec = vec![a, b, c];

    let mut ans = String::new();

    vec.into_iter().for_each(|w| {
        ans.push(w.chars().nth(0).unwrap());
    });

    ans.to_uppercase()
}

fn run2(a: &str, b: &str, c: &str) -> String {
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
    fn test() {
        let tests = [
            TestCase("atcoder", "beginner", "contest", "ABC"),
            TestCase("resident", "register", "number", "RRN"),
            TestCase("k", "nearest", "neighbor", "KNN"),
            TestCase("async", "layered", "coding", "ALC"),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
            assert_eq!(run2(a, b, c), expected);
        }
    }
}
