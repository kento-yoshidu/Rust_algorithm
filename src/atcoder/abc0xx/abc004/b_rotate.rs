// https://atcoder.jp/contests/abc004/tasks/abc004_2

fn run(s: Vec<&str>) -> Vec<String> {
    let mut ans = Vec::<String>::new();

    for v in s.iter().rev() {
        let mut row = String::new();

        for c in v.chars().rev() {
            row.push(c);
        }

        ans.push(row);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<&'static str>, Vec<&'static str>);

    #[test]
    fn test() {
        let tests = [
            TestCase(vec!["....", ".oo.", ".xx.", "...."], vec!["....", ".xx.", ".oo.", "...."]),
            TestCase(vec!["....", "....", "....", "...o"], vec!["o...", "....", "....", "...."]),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
