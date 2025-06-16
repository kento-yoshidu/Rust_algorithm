// https://atcoder.jp/contests/abc394/tasks/abc394_c

fn run(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();

    let mut i = 0;

    while i < s.len()-1 {
        if chars[i] == 'W' && chars[i+1] == 'A' {
            chars[i] = 'A';
            chars[i+1] = 'C';

            if i != 0 {
                i -= 1;
            }
        } else {
            i += 1;
        }
    }

    chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("WACWA", "ACCAC"),
            TestCase("WWA", "ACC"),
            TestCase("WWWWW", "WWWWW"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
