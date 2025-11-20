// https://atcoder.jp/contests/abc218/tasks/abc218_a

pub fn run(n: usize, s: &str) -> &'static str {
    let tmp = s.chars().nth(n-1).unwrap();

    if tmp == 'o' {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn abc218_a() {
        let tests = [
            TestCase(4,"oooxoox", "No"),
            TestCase(7,"ooooooo", "Yes"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
