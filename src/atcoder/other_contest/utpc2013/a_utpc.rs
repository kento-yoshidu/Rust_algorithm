// https://atcoder.jp/contests/utpc2013/tasks/utpc2013_01

pub fn run(s: &str) -> &'static str {
    let mut vec = Vec::new();

    for c in s.chars() {
        match c {
            'A' | 'D' | 'O' | 'P' | 'Q' | 'R' => vec.push(1),
            'B' => vec.push(2),
            _ => vec.push(0),
        }
    }

    if vec == [0, 0, 1, 0] {
        "yes"
    } else {
        "no"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("KUPC", "yes"),
            TestCase("UTPC", "yes"),
            TestCase("UTBC", "no")
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
