// https://atcoder.jp/contests/abc141/tasks/abc141_b

fn run(s: &str) -> &'static str {
    let flag = s.chars()
        .enumerate()
        .all(|(i, c)| {
            if c == 'U' || c == 'D' {
                true
            } else if i % 2 == 0 {
                c == 'R'
            } else {
                c == 'L'
            }
        });

    if flag {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCaes(&'static str, &'static str);

    #[test]
    fn abc141_b() {
        let tests = [
            TestCaes("RUDLUDR", "Yes"),
            TestCaes("DULL", "No"),
            TestCaes("UUUUUUUUUUUUUUU", "Yes"),
            TestCaes("ULURU", "No"),
            TestCaes("RDULULDURURLRDULRLR", "Yes"),
        ];

        for TestCaes(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
