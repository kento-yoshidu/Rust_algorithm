// https://atcoder.jp/contests/abc301/tasks/abc301_c

use itertools::Itertools;

pub fn run(s: &str, t: &str) -> &'static str {
    let s: Vec<char> = s.chars().sorted().collect();
    let t: Vec<char> = t.chars().sorted().collect();

    let mut s_count = s.iter().filter(|c| **c == '@').count() as isize;
    let mut t_count = t.iter().filter(|c| **c == '@').count() as isize;

    for c in 'a'..='z' {
        let mut s_char_count = 0;
        let mut t_char_count = 0;

        for sc in s.iter() {
            if c == *sc {
                s_char_count += 1;
            }
        }

        for tc in t.iter() {
            if c == *tc {
                t_char_count += 1;
            }
        }

        if s_char_count != t_char_count && !"atcoder".contains(c) {
            return "No";
        }

        if s_char_count > t_char_count {
            t_count -= s_char_count - t_char_count;
        } else {
            s_count -= t_char_count - s_char_count;
        }
    }

    if s_count < 0 || t_count < 0 {
        return "No";
    }

    "Yes"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, &'static str);

    #[test]
    fn abc301_c() {
        let tests = [
            TestCase("ch@ku@ai", "choku@@i", "Yes"),
            TestCase("ch@kud@i", "akidu@ho", "Yes"),
            TestCase("aoki", "@ok@", "No"),
            TestCase("aa", "bb", "No"),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
