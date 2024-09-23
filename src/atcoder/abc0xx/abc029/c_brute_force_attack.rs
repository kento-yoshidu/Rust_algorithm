// https://atcoder.jp/contests/abc029/tasks/abc029_c

fn func(n: usize, s: String, vec: &mut Vec<String>) -> Vec<String> {
    if n == 0 {
        vec.push(s);
        vec.clone()
    } else {
        for c in ["a", "b", "c"].iter() {
            func(n - 1, s.clone() + &c.to_string(), vec);
        }
        vec.clone()
    }
}

fn run(n: usize) -> Vec<String> {
    func(n, "".to_string(), &mut Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, vec!["a", "b", "c"]),
            TestCase(2, vec!["aa", "ab", "ac", "ba", "bb", "bc", "ca", "cb", "cc"]),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
