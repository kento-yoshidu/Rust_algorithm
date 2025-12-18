// https://paiza.jp/works/mondai/c_rank_level_up_problems/c_rank_string_step3

fn run(s: &str) -> String {
    let v: Vec<u32> = s.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    format!("{}{}", (v[0] + v[3]).to_string(), (v[1] + v[2]).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn paiza_c_level_up_02_step03() {
        let tests = [
            TestCase("2134", "64"),
            TestCase("0920", "011"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
