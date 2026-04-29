// https://atcoder.jp/contests/abc296/tasks/abc296_b

fn run(s: Vec<&str>) -> String {
    let s: Vec<Vec<char>> = s.iter().map(|s| s.chars().collect::<Vec<char>>()).collect();

    for i in 0..8 {
        for j in 0..8 {
            if s[i][j] == '*' {
                return format!("{}{}", (b'a' + j as u8)as char, 8 - i);
            }
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<&'static str>, &'static str);

    #[test]
    fn abc296_b() {
        let tests = [
            TestCase(vec!["........", "........", "........", "........", "........", "........", "........", "*......."], "a1"),
            TestCase(vec!["........", "........", "........", "........", "........", ".*......", "........", "........"], "b3"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
