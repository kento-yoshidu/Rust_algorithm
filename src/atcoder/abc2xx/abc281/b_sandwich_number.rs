// https://atcoder.jp/contests/abc281/tasks/abc281_b

fn run(s: &str) -> &'static str {
    if s.len() < 8 {
        return "No"
    }

    if !s.chars().nth(0).unwrap().is_uppercase() || !s.chars().last().unwrap().is_uppercase() {
        return "No"
    }

    let str = &s[1..s.len()-1];

    if str.chars().nth(0).unwrap() == '0' {
        return "No"
    }

    if let Ok(num) = str.parse::<usize>() {
        if 111111 <= num && num <= 999999 {
            "Yes"
        } else {
            "No"
        }
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc281_b() {
        let tests = [
            TestCase("Q142857Z", "Yes"),
            TestCase("AB912278C", "No"),
            TestCase("X900000", "No"),
            TestCase("K012345K", "No"),
            TestCase("P0123456J", "No"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
