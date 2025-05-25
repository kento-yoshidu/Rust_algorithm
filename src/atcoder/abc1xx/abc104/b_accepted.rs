// https://atcoder.jp/contests/abc104/tasks/abc104_b

fn run(s: &str) -> &'static str {
    let c: Vec<char> = s.chars().collect();

    if c[0] != 'A' {
        return "WA";
    }

    let c_count = c[2..c.len()-1].iter().filter(|&c| {
        *c == 'C'
    }).count();

    if c_count != 1 {
        return "WA";
    }

    let upper_count = c.iter().filter(|&c| {
        c.is_uppercase()
    }).count();

    if upper_count != 2 {
        return "WA";
    }

    "AC"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc104_b() {
        let tests = [
            TestCase("AtCoder", "AC"),
            TestCase("ACoder", "WA"),
            TestCase("AcycliC", "WA"),
            TestCase("AtCoCo", "WA"),
            TestCase("Atcoder", "WA"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
