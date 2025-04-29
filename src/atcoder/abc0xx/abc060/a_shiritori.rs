// https://atcoder.jp/contests/abc060/tasks/abc060_a

fn run(a: &str, b: &str, c: &str) -> &'static str {
    let ar = a.chars().last().unwrap();
    let bl = b.chars().nth(0).unwrap();
    let br = b.chars().last().unwrap();
    let cl = c.chars().nth(0).unwrap();

    if ar == bl && br == cl {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("rng", "gorilla", "apple", "Yes"),
            TestCase("yakiniku", "unagi", "sushi", "No"),
            TestCase("a", "a", "a", "Yes"),
            TestCase("aaaaaaaaab", "aaaaaaaaaa aaaaaaaaab", "a", "No"),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
