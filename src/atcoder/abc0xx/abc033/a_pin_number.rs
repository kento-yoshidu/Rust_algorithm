// https://atcoder.jp/contests/abc033/tasks/abc033_a

fn run(n: &str) -> &'static str {
    let chars: Vec<char> = n.chars().collect();

    if chars[0] == chars[1] && chars[2] == chars[3] && chars[0] == chars[3] {
        "SAME"
    } else {
        "DIFFERENT"
    }
}

fn run2(n: &str) -> &'static str {
    use std::collections::HashSet;

    let map: HashSet<char> = n.chars().collect();

    if map.len() == 1 {
        "SAME"
    } else {
        "DIFFERENT"
    }
}

fn run3(n: &str) -> &'static str {
    let mut vec: Vec<char> = n.chars().collect();

    vec.dedup();

    if vec.len() == 1 {
        "SAME"
    } else {
        "DIFFERENT"
    }
}

fn run4(n: usize) -> &'static str {
    if n % 1111 == 0 {
        "SAME"
    } else {
        "DIFFERENT"
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
            assert_eq!(run4(n.parse().unwrap()), expected);
        }
    }
}
