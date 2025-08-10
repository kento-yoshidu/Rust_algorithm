// https://paiza.jp/works/mondai/d_rank_level_up_problems/d_rank_level_up_problems__accompanied_by_stdin_1

fn run(s: &str, t: &str) -> String {
    format!("{s}\n{t}")
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, &'static str);

    #[test]
    fn paiza_d_level_up_04_step01() {
        let tests = [
            TestCase("paiza", "learning", "paiza\nlearning"),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
