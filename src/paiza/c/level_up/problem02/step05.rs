// https://paiza.jp/works/mondai/c_rank_level_up_problems/c_rank_string_step5

fn run(s: &str) -> (String, String) {
    let parts: Vec<_> = s.split(':').collect();

    let h = parts[0].trim_start_matches('0');
    let m = parts[1].trim_start_matches('0');

   (
        if h.is_empty() { "0".to_string() } else { h.to_string() },
        if m.is_empty() { "0".to_string() } else { m.to_string() },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, (String, String));

    #[test]
    fn paiza_c_level_up_02_step05() {
        let tests = [
            TestCase("12:34", ("12".to_string(), "34".to_string())),
            TestCase("01:03", ("1".to_string(), "3".to_string())),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
