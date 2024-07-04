// https://atcoder.jp/contests/abc360/tasks/abc360_b

pub fn run(s: &str, t: &str) -> &'static str {
    for w in 1..s.len() {
        for c in 0..w {
            let mut str = String::new();

            let mut index = c;

            while index < s.len() {
                str.push(s.chars().nth(index).unwrap());
                index += w;
            }

            if str == t {
                return "Yes"
            }
        }
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("atcoder", "toe", "Yes"),
            TestCase("beginner", "r", "No"),
            TestCase("verticalreading", "agh", "No"),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t) ,expected);
        }

    }
}
