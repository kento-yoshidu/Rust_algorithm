// https://atcoder.jp/contests/abc132/tasks/abc132_a

fn run(s: &str) -> &'static str {
    let mut temp: Vec<_> = s.chars().collect();

    temp.sort();

    if temp[0] == temp[1] && temp[2] == temp[3] && temp[1] != temp[2] {
        "Yes"
    } else {
        "No"
    }
}

pub fn run2(s: &str) -> &'static str {
    let mut vec: Vec<char> = s.chars().collect();

    vec.sort();
    vec.dedup();

    if vec.len() == 2 {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc132_a() {
        let tests = [
            TestCase("ASSA", "Yes"),
            TestCase("STOP", "No"),
            TestCase("FFEE", "Yes"),
            TestCase("FREE", "No"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
            assert_eq!(run2(s), expected);
        }
    }
}
