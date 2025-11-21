// https://yukicoder.me/problems/no/1217

fn run(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();

    for (i, c) in ('a'..='z').enumerate() {
        if chars[i] != c {
            return format!("{}to{}", c, chars[i]);
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn yuki_1217() {
        let tests = [
            TestCase("abcdefghijklmnopqrituvwxyz", "stoi"),
            TestCase("abcdefghujklmnopqrstuvwxyz", "itou"),
            TestCase("abcdefghijklmnnpqrstuvwxyz", "oton"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
