// https://atcoder.jp/contests/abc033/tasks/abc033_a

pub fn run(n: &str) -> String {
    let chars: Vec<char> = n.chars().collect();

    if chars[0] == chars[1] && chars[2] == chars[3] && chars[0] == chars[3] {
        String::from("SAME")
    } else {
        String::from("DIFFERENT")
    }
}

pub fn run2(n: &str) -> String {
    use std::collections::HashSet;

    let map: HashSet<char> = n.chars().collect();

    if map.len() == 1 {
        String::from("SAME")
    } else {
        String::from("DIFFERENT")
    }
}

pub fn run3(n: &str) -> String {
    let mut vec: Vec<char> = n.chars().collect();

    vec.dedup();

    if vec.len() == 1 {
        String::from("SAME")
    } else {
        String::from("DIFFERENT")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("2222", "SAME"),
            TestCase("1221", "DIFFERENT"),
            TestCase("0000", "SAME"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
            assert_eq!(run2(n), expected);
            assert_eq!(run3(n), expected);
        }
    }
}
