// https://yukicoder.me/problems/no/2960

fn run(s1: &str, s2: &str, s3: &str, s4: &str, s5: &str, s6: &str, s7: &str, s8: &str) -> &'static str {
    let count = [s1, s2, s3, s4, s5, s6, s7, s8]
        .into_iter()
        .filter(|str| *str == "AC" || *str == "NoOut")
        .count();

    if count >= 6 {
        "Win"
    } else if count == 5 {
        "Draw"
    } else {
        "Lose"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str);

    #[test]
    fn yuki_2960() {
        let tests = [
            TestCase("AC", "AC", "AC", "WA", "NoOut", "NoSub", "WA", "NoOut", "Draw"),
            TestCase("AC", "WA", "WA", "AC", "AC", "AC", "AC", "AC", "Win"),
            TestCase("AC", "WA", "WA", "NoSub", "NoSub", "RE", "AC", "RE", "Lose"),
        ];

        for TestCase(s1, s2,s3, s4, s5, s6, s7, s8, expected) in tests {
            assert_eq!(run(s1, s2, s3, s4, s5, s6, s7, s8), expected);
        }
    }
}
