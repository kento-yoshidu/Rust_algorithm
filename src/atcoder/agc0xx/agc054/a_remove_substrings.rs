// https://atcoder.jp/contests/agc054/tasks/agc054_a

fn run(n: usize, s: &str) -> isize {
    let chars: Vec<char> = s.chars().collect();

    if chars[0] != *chars.iter().last().unwrap() {
        return 1;
    }

    for i in 1..n-1 {
        if chars[0] != chars[i] && chars[0] != chars[i+1] {
            return 2;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, "abba", 2),
            TestCase(3, "aba", -1),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
