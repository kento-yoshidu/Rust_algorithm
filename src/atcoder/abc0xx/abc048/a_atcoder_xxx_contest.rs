// https://atcoder.jp/contests/abc048/tasks/abc048_a

pub fn run(a: &str, b: &str, c: &str) -> String {
    let vec = vec![a, b, c];

    let mut ans = String::new();

    vec.iter().for_each(|s| {
        ans.push(s.chars().nth(0).unwrap());
    });

    ans
}

pub fn run2(a: &str, b: &str, c: &str) -> String {
    let vec = vec![a, b, c];

    vec.iter()
        .map(|v| {
            v.chars().nth(0).unwrap()
        })
        .collect()
}

pub fn run3(_a: &str, b: &str, _c: &str) -> String {
    format!("A{}C", b.chars().nth(0).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("AtCoder", "Beginner", "Contest", "ABC"),
            TestCase("AtCoder", "Snuke", "Contest", "ASC"),
            TestCase("AtCoder", "X", "Contest", "AXC"),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
            assert_eq!(run2(a, b, c), expected);
            assert_eq!(run3(a, b, c), expected);
        }
    }
}
