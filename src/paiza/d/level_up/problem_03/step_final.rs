// https://paiza.jp/works/mondai/d_rank_level_up_problems/d_rank_level_up_problems__stdin_boss

fn run(s: &str, t: &str) -> String {
    format!("{s}\n{t}")
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, &'static str);

    #[test]
    fn paiza_d_level_up_03_step_final() {
        let tests = [
            TestCase("hello", "paiza", "hello\npaiza"),
            TestCase("81", "3", "81\n3"),
            TestCase("hello", "813", "hello\n813"),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
