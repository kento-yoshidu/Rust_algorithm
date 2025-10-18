// https://yukicoder.me/problems/no/3036

fn run(n: usize, s: &str) -> (&'static str, Option<(String, String)>) {
    if n % 2 != 0 {
        return ("No", None);
    }

    let mut x = String::new();
    let mut y = String::new();

    for (i, c) in s.chars().enumerate() {
        if i % 2 == 0 {
            x.push(c);
        } else {
            y.push(c);
        }
    }

    ("Yes", Some((x, y)))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, (&'static str, Option<(String, String)>));

    #[test]
    fn yuki_3036() {
        let tests = [
            TestCase(8, "nauclhlt", ("Yes", Some(("null".to_string(), "acht".to_string())))),
            TestCase(5, "apple", ("No", None)),
            TestCase(6, "ababab", ("Yes", Some(("aaa".to_string(), "bbb".to_string())))),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
